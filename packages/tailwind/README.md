## 使用指南 Get Started

1. `npm i huamei-colors` or `yarn add huamei-colors`
2. 拓展 tailwind 的主题颜色:
```js
// tailwind.config.js
const { colors } = require('@huamei/colors')

module.exports = {
  theme: {
    extend: {...colors},
  },
  ...
}
```
3. 使用颜色，比如 `text-renlai`
