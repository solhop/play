# SolHOP Playground

[![Netlify Status](https://api.netlify.com/api/v1/badges/ba2688ec-69cd-4827-a2d6-973df7e81e4e/deploy-status)](https://app.netlify.com/sites/solhop-play/deploys)
![.github/workflows/test.yml](https://github.com/solhop/play/workflows/.github/workflows/test.yml/badge.svg)
![.github/workflows/publish.yml](https://github.com/solhop/play/workflows/.github/workflows/publish.yml/badge.svg)

A playground to test out the tools.

---

## How to install

```sh
npm install
```

## How to run in debug mode

```sh
# Builds the project and opens it in a new browser tab. Auto-reloads when the project changes.
npm start
```

## How to build in release mode

```sh
# Builds the project and places it into the `dist` folder.
npm run build
```

## How to run unit tests

```sh
# Runs tests in Firefox
npm test -- --firefox

# Runs tests in Chrome
npm test -- --chrome

# Runs tests in Safari
npm test -- --safari
```

---

LICENSE: [MIT](LICENSE)
