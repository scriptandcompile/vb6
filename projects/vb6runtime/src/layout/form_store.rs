//! Mutable state store for loaded VB6 forms.
//!
//! Manages [`LayoutForm`] instances via incrementing [`FormHandle`] IDs.
//! Uses a process-global static store protected by a `Mutex`.
//!
//! # Process-Global Store
//!
//! All forms are stored in a static `Mutex<HashMap>`. Handles are never
//! freed in Phase 1 (memory leak is acceptable for now).
//!
//! # API Design
//!
//! [`get`] and [`get_mut`] use a closure-based pattern to avoid the
//! `'static` reference lifetime problem inherent in global mutable state
//! protected by a `Mutex`. The closure is invoked while the lock is held,
//! giving the caller mutable access without returning a dangling reference.
//!
//! ```
//! // Instead of:
//! // let form = store.get(handle);  // Can't return 'static ref safely
//! // form.caption = "Hi";          // Dangling reference
//!
//! // Use:
//! // store.get_mut(handle, |form| {
//! //     form.caption = "Hi";
//! // });
//! ```

use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

use super::model::LayoutForm;

/// Handle type for referencing a loaded form in the store.
///
/// Handles are incrementing u32 values starting from 0.
/// They remain valid for the lifetime of the process.
pub type FormHandle = u32;

/// The global form store.
///
/// Initialized lazily on first `insert` to avoid unnecessary static
/// allocation when no forms are loaded.
static STORE: LazyLock<Mutex<HashMap<FormHandle, LayoutForm>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Next handle ID to assign.
///
/// Stored separately from the store so that handle allocation doesn't
/// conflict with form data access. Actually, we can store it inside
/// the store. Let me reconsider...
///
/// Actually, we need to track the next handle ID. Since the store is
/// a `LazyLock<Mutex<...>>`, we need to be able to access it on first
/// insert. The simplest approach is to use a separate static for the
/// counter. But that creates a two-static pattern which is messy.
///
/// Better approach: store the counter inside the HashMap as a special key.
/// But that's hacky.
///
/// Simplest correct approach: just start from 0 and track it in the
/// HashMap itself. When we insert, scan for the max key and add 1.
/// This is O(n) for handle allocation but forms are few in Phase 1.
///
/// Actually, the cleanest approach is to store both counter and forms
/// in the HashMap. But HashMap can't hold different types.
///
/// Final approach: use a separate LazyLock for the counter.
static NEXT_HANDLE: LazyLock<Mutex<FormHandle>> = LazyLock::new(|| Mutex::new(0));

/// Insert a form into the store and return its handle.
///
/// The handle is an incrementing u32 starting from 0.
pub fn insert(form: LayoutForm) -> FormHandle {
    let mut handle = NEXT_HANDLE.lock().unwrap();
    let current = *handle;
    *handle += 1;
    drop(handle);

    STORE.lock().unwrap().insert(current, form);
    current
}

/// Operate on a form by handle via a closure.
///
/// The closure receives an immutable reference to the form while the
/// store lock is held. Returns the closure's result.
///
/// Returns `None` if the handle is invalid.
pub fn get<T>(handle: FormHandle, f: impl FnOnce(&LayoutForm) -> T) -> Option<T> {
    let store = STORE.lock().unwrap();
    store.get(&handle).map(f)
}

/// Operate on a form by handle via a closure.
///
/// The closure receives a mutable reference to the form while the
/// store lock is held. Returns the closure's result.
///
/// Returns `None` if the handle is invalid.
pub fn get_mut<T>(handle: FormHandle, f: impl FnOnce(&mut LayoutForm) -> T) -> Option<T> {
    let mut store = STORE.lock().unwrap();
    store.get_mut(&handle).map(f)
}

/// Remove a form from the store by handle.
///
/// Returns `Some(form)` if the form was found and removed, `None` otherwise.
pub fn remove(handle: FormHandle) -> Option<LayoutForm> {
    STORE.lock().unwrap().remove(&handle)
}

/// Reset the store to its initial empty state.
///
/// This function is `pub(crate)` to support test isolation within the
/// `vb6runtime` crate. In production code, the store's lifetime matches
/// the process lifetime.
#[cfg(test)]
pub(crate) fn reset() {
    STORE.lock().unwrap().clear();
    *NEXT_HANDLE.lock().unwrap() = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_form(name: &str) -> LayoutForm {
        LayoutForm {
            name: name.into(),
            ..LayoutForm::default()
        }
    }

    #[test]
    fn store_insert_and_get() {
        let form = test_form("Form1");
        let handle = insert(form);
        let retrieved = get(handle, |f| f.name.clone());
        assert_eq!(retrieved, Some("Form1".into()));
    }

    #[test]
    fn store_get_mut() {
        let form = test_form("Form1");
        let handle = insert(form);
        let modified = get_mut(handle, |f| {
            f.caption = "Modified".into();
            f.caption.clone()
        });
        assert_eq!(modified, Some("Modified".into()));
        let check = get(handle, |f| f.caption.clone());
        assert_eq!(check, Some("Modified".into()));
    }

    #[test]
    fn store_invalid_handle() {
        let got = get(999, |f| f.name.clone());
        assert_eq!(got, None);
        let got_mut = get_mut(999, |f| f.name.clone());
        assert_eq!(got_mut, None);
    }

    #[test]
    fn store_incrementing_handles() {
        let f1 = insert(LayoutForm {
            name: "Form1".into(),
            ..LayoutForm::default()
        });
        let f2 = insert(LayoutForm {
            name: "Form2".into(),
            ..LayoutForm::default()
        });
        let f3 = insert(LayoutForm {
            name: "Form3".into(),
            ..LayoutForm::default()
        });
        assert!(f1 < f2);
        assert!(f2 < f3);
        let names: Vec<_> = [f1, f2, f3]
            .iter()
            .map(|&h| get(h, |f| f.name.clone()))
            .collect();
        assert_eq!(
            names,
            vec![
                Some("Form1".into()),
                Some("Form2".into()),
                Some("Form3".into()),
            ]
        );
    }

    #[test]
    fn store_remove() {
        let form = test_form("Form1");
        let handle = insert(form);
        let removed = remove(handle);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().name, "Form1");
        let got = get(handle, |f| f.name.clone());
        assert_eq!(got, None);
    }

    #[test]
    fn store_remove_invalid() {
        let result = remove(999);
        assert!(result.is_none());
    }

    #[test]
    fn store_multiple_forms_independent() {
        let f1 = insert(LayoutForm {
            name: "Form1".into(),
            visible: true,
            ..LayoutForm::default()
        });
        let f2 = insert(LayoutForm {
            name: "Form2".into(),
            visible: false,
            ..LayoutForm::default()
        });

        let v1 = get(f1, |f| f.visible);
        let v2 = get(f2, |f| f.visible);
        assert_eq!(v1, Some(true));
        assert_eq!(v2, Some(false));

        get_mut(f1, |f| {
            f.visible = false;
        });
        let v1 = get(f1, |f| f.visible);
        let v2 = get(f2, |f| f.visible);
        assert_eq!(v1, Some(false));
        assert_eq!(v2, Some(false));
    }
}
