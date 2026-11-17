use std::collections::BTreeMap;

use crate::value::VBVariant;
use crate::{VBError, VBResult};

/// Stable identifier for a runtime-owned value cell.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ValueSlot(u64);

impl ValueSlot {
    /// Returns the stable numeric identifier.
    pub const fn value(self) -> u64 {
        self.0
    }
}

/// Opaque alias to a value cell, safe to pass between interpreter frames.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ByRefHandle {
    slot: ValueSlot,
}

impl ByRefHandle {
    /// Returns the aliased slot identifier.
    pub const fn slot(self) -> ValueSlot {
        self.slot
    }
}

/// Owns runtime values addressed through stable slots and explicit ByRef handles.
#[derive(Debug, Default)]
pub struct ValueStore {
    next_slot: u64,
    values: BTreeMap<ValueSlot, VBVariant>,
}

impl ValueStore {
    /// Allocates a stable slot containing `value`.
    pub fn insert(&mut self, value: VBVariant) -> VBResult<ValueSlot> {
        let next = self
            .next_slot
            .checked_add(1)
            .ok_or_else(VBError::overflow)?;
        let slot = ValueSlot(next);
        self.values.insert(slot, value);
        self.next_slot = next;
        Ok(slot)
    }

    /// Returns a clone of the value in `slot`.
    pub fn read(&self, slot: ValueSlot) -> VBResult<VBVariant> {
        self.values
            .get(&slot)
            .cloned()
            .ok_or_else(VBError::invalid_procedure_call)
    }

    /// Replaces the value in `slot`.
    pub fn write(&mut self, slot: ValueSlot, value: VBVariant) -> VBResult<()> {
        let stored = self
            .values
            .get_mut(&slot)
            .ok_or_else(VBError::invalid_procedure_call)?;
        *stored = value;
        Ok(())
    }

    /// Removes `slot`, invalidating all aliases to it.
    pub fn release(&mut self, slot: ValueSlot) -> VBResult<()> {
        self.values
            .remove(&slot)
            .map(|_| ())
            .ok_or_else(VBError::invalid_procedure_call)
    }

    /// Creates an alias to a live slot.
    pub fn by_ref(&self, slot: ValueSlot) -> VBResult<ByRefHandle> {
        if !self.values.contains_key(&slot) {
            return Err(VBError::invalid_procedure_call());
        }
        Ok(ByRefHandle { slot })
    }

    /// Reads the current value through an alias.
    pub fn read_by_ref(&self, handle: ByRefHandle) -> VBResult<VBVariant> {
        self.read(handle.slot)
    }

    /// Replaces the value addressed by an alias.
    pub fn write_by_ref(&mut self, handle: ByRefHandle, value: VBVariant) -> VBResult<()> {
        self.write(handle.slot, value)
    }

    /// Returns whether `slot` is currently allocated.
    pub fn contains(&self, slot: ValueSlot) -> bool {
        self.values.contains_key(&slot)
    }
}

#[cfg(test)]
mod tests {
    use super::ValueStore;
    use crate::VBVariant;

    #[test]
    fn byref_handle_aliases_a_live_slot_and_expires_on_release() {
        let mut store = ValueStore::default();
        let slot = store.insert(VBVariant::Long(7)).unwrap();
        let alias = store.by_ref(slot).unwrap();

        store.write_by_ref(alias, VBVariant::Long(11)).unwrap();
        assert_eq!(store.read(slot).unwrap(), VBVariant::Long(11));
        assert_eq!(store.read_by_ref(alias).unwrap(), VBVariant::Long(11));

        store.release(slot).unwrap();
        assert!(store.read_by_ref(alias).is_err());
        let next = store.insert(VBVariant::Long(13)).unwrap();
        assert_ne!(slot, next);
    }

    #[test]
    fn released_slots_reject_all_further_operations() {
        let mut store = ValueStore::default();
        let slot = store.insert(VBVariant::Empty).unwrap();
        let alias = store.by_ref(slot).unwrap();
        store.release(slot).unwrap();
        assert!(store.read(slot).is_err());
        assert!(store.by_ref(slot).is_err());
        assert!(store.write_by_ref(alias, VBVariant::Null).is_err());
        assert!(store.release(slot).is_err());
    }
}
