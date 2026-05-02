module.exports = {
  extends: ['stylelint-config-standard-scss'],
  rules: {
    'selector-class-pattern':
      '^[a-z]+(?:-[a-z]+)*(?:__(?:[a-z]+(?:-[a-z]+)*))?(?:--(?:[a-z]+(?:-[a-z]+)*))?$',
  },
};
