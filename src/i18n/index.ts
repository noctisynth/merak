import i18n from 'i18next';
import { initReactI18next } from 'react-i18next';

// import LanguageDetector from 'i18next-browser-languagedetector'
// Auto language detection (disabled for now)

// Auto-import all JSON files under en/ and zh/
const enModules: Record<string, { default: Record<string, unknown> }> =
  import.meta.glob('./en/*.json', { eager: true });
const zhModules = import.meta.glob('./zh/*.json', { eager: true }) as Record<
  string,
  { default: Record<string, unknown> }
>;

// Convert modules into namespace-based resource objects
const resources = {
  en: Object.fromEntries(
    Object.entries(enModules).map(([path, mod]) => {
      const ns = path.split('/').pop()?.replace('.json', '') ?? path;
      return [ns, mod.default];
    }),
  ),
  zh: Object.fromEntries(
    Object.entries(zhModules).map(([path, mod]) => {
      const ns = path.split('/').pop()?.replace('.json', '') ?? path;
      return [ns, mod.default];
    }),
  ),
};

// Initialize i18n
i18n
  // .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: 'en',
    ns: Object.keys(resources.en),
    defaultNS: 'landing',
    interpolation: {
      escapeValue: false,
    },
    react: {
      useSuspense: false,
    },
  });

export default i18n;
