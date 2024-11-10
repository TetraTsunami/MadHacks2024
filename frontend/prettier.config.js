/**
 * @see https://prettier.io/docs/en/configuration.html
 * @type {import('prettier').Config}
 */
const config = {
  trailingComma: 'none',
  tabWidth: 2,
  semi: false,
  singleQuote: true,
  endOfLine: 'auto', // This is solved by Git and .gitattributes, so just use whatever the file uses
  plugins: [
    'prettier-plugin-svelte',
    'prettier-plugin-tailwindcss' // MUST come last - https://github.com/tailwindlabs/prettier-plugin-tailwindcss?tab=readme-ov-file#compatibility-with-other-prettier-plugins
  ],
  overrides: [{ files: '*.svelte', options: { parser: 'svelte' } }]
}

export default config
