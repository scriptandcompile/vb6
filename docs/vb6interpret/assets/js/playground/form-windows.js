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

        const container = document.createElement('div');
        container.id = containerId;

        // Verify the element exists in the DOM before calling WASM
        if (!document.getElementById(containerId)) {
            document.body.appendChild(container);
        }

        const handle = await window.show_form(formBytes, containerId);

        const bindings = await window.get_form_procedures(handle);
        const caption = await window.get_form_caption(handle);

        const win = new FormWindow(container, bindings, stateHandle, containerId, caption);
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

    dispatchEvent(stateHandle, controlName, eventName) {
        return window.form_event(stateHandle, controlName, eventName);
    }

    _nextZIndex() {
        return ++this._zIndexCounter;
    }
}

class FormWindow {
    constructor(contentEl, bindings = [], stateHandle = null, containerId = 'vb6-container', caption = null) {
        this.bindings = bindings;
        this.stateHandle = stateHandle;
        this.containerId = containerId;
        this.formHandle = null;

        this.el = document.createElement('div');
        this.el.className = 'vb6-form-window';
        this.el.style.left = '20px';
        this.el.style.top = '120px';
        this.el.style.zIndex = formManager._nextZIndex();

        const titlebar = document.createElement('div');
        titlebar.className = 'vb6-form-titlebar';

        const title = document.createElement('span');
        title.className = 'vb6-form-title';
        title.textContent = caption || 'Form';
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
        let allChildren = [];
        function walk(node, depth) {
            if (!node) return;
            if (depth < 4) {
                allChildren.push(depth + ': ' + node.tagName + (node.id ? '#'+node.id : '') + (node.className ? '.'+node.className : '') + (node.textContent ? ' "'+node.textContent.substring(0,20)+'"' : ''));
            }
            for (let i = 0; i < node.children.length; i++) {
                walk(node.children[i], depth + 1);
            }
        }
        walk(contentEl, 0);
        for (const binding of this.bindings) {
            const control = binding.get('control');
            const event = binding.get('event');
            const el = contentEl.querySelector('[id="' + control + '"]');
            if (!el) continue;
            const handler = () => {
                if (el.disabled) return;
                if (this.stateHandle === null) return;
                try {
                    const result = formManager.dispatchEvent(
                        this.stateHandle,
                        control,
                        event
                    );
                    if (window.renderOutput) {
                        window.renderOutput(result);
                    }
                } catch (e) {
                    console.error('form-windows: error dispatching event:', e);
                    if (window.renderOutput) {
                        window.renderOutput({ successful: false, output_text: '', output_lines: [], steps: 0, terminated: false, error: { message: e.message ?? 'Execution failed.' }, debug: { current_steps: 0, current_line: 1, current_procedure: null, stack_depth: 0, globals: [], locals: [], cursor: null } });
                    }
                }
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
    window.formManager = new FormWindowManager();
    formManager = window.formManager;
});
