import { useTranslation } from 'react-i18next';

export type Lang = 'en' | 'zh';

const LANG_LABEL: Record<Lang, string> = {
  en: 'EN',
  zh: '中文',
};

export function useLocale() {
  const { i18n } = useTranslation();

  const current = (i18n.language.startsWith('zh') ? 'zh' : 'en') as Lang;

  const change = (lang: Lang) => {
    i18n.changeLanguage(lang);
  };

  const toggle = () => {
    change(current === 'en' ? 'zh' : 'en');
  };

  return {
    lang: current, // current language
    label: LANG_LABEL[current], // Button label

    change,
    toggle,
  };
}
