import js from '@eslint/js'
import vue from 'eslint-plugin-vue'
import ts from 'typescript-eslint'
import prettier from 'eslint-config-prettier'

export default [
  js.configs.recommended,
  ...ts.configs.recommended,
  ...vue.configs['flat/recommended'],
  {
    files: ['src/**/*.{ts,vue}'],
    languageOptions: {
      globals: {
        window: 'readonly',
        document: 'readonly',
        navigator: 'readonly',
        HTMLElement: 'readonly',
        HTMLInputElement: 'readonly',
        HTMLButtonElement: 'readonly',
        KeyboardEvent: 'readonly',
        MouseEvent: 'readonly',
        requestAnimationFrame: 'readonly',
        cancelAnimationFrame: 'readonly',
        performance: 'readonly',
        setInterval: 'readonly',
        clearInterval: 'readonly',
      },
      parserOptions: {
        parser: ts.parser,
      },
    },
    rules: {
      'vue/multi-word-component-names': 'off',
      'vue/no-v-html': 'off',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_' }],
      'no-empty': ['error', { allowEmptyCatch: true }],
      'curly': ['error', 'all'],
    },
  },
  prettier,
  {
    ignores: ['dist/**', 'node_modules/**', '*.config.*'],
  },
]
