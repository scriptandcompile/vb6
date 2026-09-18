/**
 * FormWindowManager — creates, positions, and destroys VB6 form
 * window overlays on top of the playground.
 *
 * Each form is rendered into its own absolutely-positioned <div>
 * with a title bar (caption + close button) and a content area
 * that holds the vb6runtime layout DOM tree.
 *
 * Per-window containers: each form gets its own #vb6-container-N element.
 */

let formManager;

function makeDraggable(windowEl) {
    const titlebar = windowEl.querySelector('.vb6-form-titlebar');
    let isDragging = false;
    let startX, startY, origX, origY;

    titlebar.addEventListener('mousedown', (e) => {
        if (e.target.classList.contains('vb6-form-close')) return;
        isDragging = true;
        startX = e.clientX;
        startY = e.clientY;
        origX = parseInt(windowEl.style.left) || 0;
        origY = parseInt(windowEl.style.top) || 0;
        windowEl.style.zIndex = formManager._nextZIndex();
    });

    document.addEventListener('mousemove', (e) => {
        if (!isDragging) return;
        windowEl.style.left = (origX + e.clientX - startX) + 'px';
        windowEl.style.top = (origY + e.clientY - startY) + 'px';
    });

    document.addEventListener('mouseup', () => {
        isDragging = false;
    });
}

class FormWindowManager {
    constructor() {
        this.windows = new Map();
        this._zIndexCounter = 1;
        this._formIndex = 0;
    }

    async showForm(formBytes, stateHandle = null) {
        const containerId = `vb6-container-${this._formIndex++}`;

        const handle = await window.show_form(formBytes, containerId);

        const bindings = await window.get_form_procedures(handle);

        const container = document.createElement('div');
        container.id = containerId;

        const win = new FormWindow(container, bindings, stateHandle, containerId);
        win.formHandle = handle;
        this.windows.set(handle, win);

        return { handle, bindings, containerId };
    }

    unloadForm(handle, containerId) {
        const win = this.windows.get(handle);
        if (win) {
            win.destroy();
            this.windows.delete(handle);
        }
        window.unload_form(handle, containerId);
    }

    dispatchEvent(stateHandle, controlName, eventName, procedureName) {
        return window.call_sub(stateHandle, procedureName);
    }

    _nextZIndex() {
        return ++this._zIndexCounter;
    }
}

class FormWindow {
    constructor(contentEl, bindings = [], stateHandle = null, containerId = 'vb6-container') {
        this.bindings = bindings;
        this.stateHandle = stateHandle;
        this.containerId = containerId;
        this.formHandle = null;

        this.el = document.createElement('div');
        this.el.className = 'vb6-form-window';
        this.el.style.left = '50px';
        this.el.style.top = '50px';
        this.el.style.zIndex = formManager._nextZIndex();

        const titlebar = document.createElement('div');
        titlebar.className = 'vb6-form-titlebar';

        const title = document.createElement('span');
        title.className = 'vb6-form-title';
        title.textContent = 'Form';
        this.titleEl = title;

        const closeBtn = document.createElement('button');
        closeBtn.className = 'vb6-form-close';
        closeBtn.title = 'Close';
        closeBtn.textContent = '\u2715';
        closeBtn.addEventListener('click', () => {
            this.destroy();
        });

        titlebar.appendChild(title);
        titlebar.appendChild(closeBtn);

        const content = document.createElement('div');
        content.className = 'vb6-form-content';
        content.appendChild(contentEl);

        this.el.appendChild(titlebar);
        this.el.appendChild(content);

        let container = document.getElementById('form-windows-container');
        if (!container) {
            container = document.createElement('div');
            container.id = 'form-windows-container';
            document.body.appendChild(container);
        }
        container.appendChild(this.el);

        this._attachBindings(contentEl);

        makeDraggable(this.el);
    }

    _attachBindings(contentEl) {
        for (const binding of this.bindings) {
            const el = contentEl.getElementById(binding.control);
            if (!el) continue;

            const handler = () => {
                if (el.disabled) return;
                if (this.stateHandle === null) return;
                formManager.dispatchEvent(
                    this.stateHandle,
                    binding.control,
                    binding.event,
                    binding.procedure
                );
            };

            const eventName = this._toDomEvent(binding.event);
            el.addEventListener(eventName, handler);
        }
    }

    _toDomEvent(vb6Event) {
        const map = {
            'Click': 'click',
            'DblClick': 'dblclick',
            'MouseDown': 'mousedown',
            'MouseMove': 'mousemove',
            'MouseUp': 'mouseup',
            'Change': 'input',
            'GotFocus': 'focus',
            'LostFocus': 'blur',
            'KeyPress': 'keypress',
            'KeyDown': 'keydown',
            'KeyUp': 'keyup',
            'DragDrop': 'drop',
            'DragOver': 'dragover',
            'Paint': 'paint',
        };
        return map[vb6Event] || 'click';
    }

    setTitle(caption) {
        this.titleEl.textContent = caption;
    }

    async destroy() {
        const container = document.getElementById(this.containerId);
        if (container && container.parentNode) {
            container.parentNode.removeChild(container);
        }

        try {
            await window.unload_form(this.formHandle, this.containerId);
        } catch (e) {
            console.warn('Failed to unload form in WASM:', e);
        }

        if (this.el.parentNode) {
            this.el.parentNode.removeChild(this.el);
        }

        if (this.stateHandle !== null) {
            window.dispose_state(this.stateHandle);
        }
    }
}

document.addEventListener('DOMContentLoaded', () => {
    formManager = new FormWindowManager();
});
