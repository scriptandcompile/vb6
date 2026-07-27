// Theme switcher for the VB6 workspace docs.
(function() {
    const THEME_KEY = 'vb6-workspace-theme';

    function getSystemPreference() {
        if (window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches) {
            return 'dark';
        }

        return 'light';
    }

    function getTheme() {
        const savedTheme = localStorage.getItem(THEME_KEY);
        if (savedTheme) {
            return savedTheme;
        }

        return getSystemPreference();
    }

    function saveTheme(theme) {
        localStorage.setItem(THEME_KEY, theme);
    }

    function applyTheme(theme) {
        document.documentElement.setAttribute('data-theme', theme);

        const icon = document.querySelector('.theme-icon');
        if (icon) {
            icon.textContent = theme === 'dark' ? '☀️' : '🌙';
        }
    }

    function toggleTheme() {
        const nextTheme = getTheme() === 'dark' ? 'light' : 'dark';
        saveTheme(nextTheme);
        applyTheme(nextTheme);
    }

    const initialTheme = getTheme();
    document.documentElement.setAttribute('data-theme', initialTheme);

    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            applyTheme(initialTheme);
            const toggleButton = document.getElementById('theme-toggle');
            if (toggleButton) {
                toggleButton.addEventListener('click', toggleTheme);
            }
        });
    } else {
        applyTheme(initialTheme);
        const toggleButton = document.getElementById('theme-toggle');
        if (toggleButton) {
            toggleButton.addEventListener('click', toggleTheme);
        }
    }
})();