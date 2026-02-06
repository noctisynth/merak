import type { ButtonHTMLAttributes, ReactNode } from 'react';
import { useTranslation } from 'react-i18next';
import { Link } from 'react-router';
import { GithubIcon, MerakIcon, MerakLogo } from '../../public/icon';
import { useLocale } from '../hooks/useLocale';

interface Props
  extends Omit<ButtonHTMLAttributes<HTMLButtonElement>, 'onClick'> {
  action?: () => void;
  children: ReactNode;
}

export const NavActionButton = ({
  className = '',
  children,
  action,
  ...rest
}: Props) => {
  const base =
    'h-9 min-w-9 px-3 flex items-center justify-center rounded-lg border border-gray-200 bg-white shadow-sm transition-all duration-200 hover:bg-gray-100 active:scale-95';

  return (
    <button className={`${base} ${className}`} onClick={action} {...rest}>
      {children}
    </button>
  );
};

export default function LandingPage() {
  const { t } = useTranslation('landing');
  const { lang, label, toggle } = useLocale();
  return (
    <div className="relative min-h-screen overflow-hidden bg-white text-gray-900">
      <header className="fixed top-0 w-full z-50 backdrop-blur-xl bg-white/70 border-b border-gray-200">
        <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 flex items-center justify-center rounded-lg bg-linear-to-br from-indigo-600 to-blue-600 shadow-sm">
              <MerakIcon className="w-5 h-5 text-white" />
            </div>

            <span className="font-semibold text-lg tracking-tight">
              {t('brandName')}
            </span>
          </div>

          {/* Decorative background glow */}
          <div className="absolute inset-0 -z-10 bg-linear-to-br from-indigo-50 via-white to-blue-50" />
          <div className="absolute -top-40 -right-40 w-[520px] h-[520px] bg-indigo-500/30 rounded-full blur-[120px] -z-10" />
          <div className="absolute bottom-0 -left-40 w-[520px] h-[520px] bg-blue-500/30 rounded-full blur-[120px] -z-10" />

          <div className="flex items-center gap-3">
            <NavActionButton
              aria-label="Toggle theme"
              className="cursor-pointer"
            >
              🌙
            </NavActionButton>

            <NavActionButton
              action={toggle}
              aria-label={
                lang === 'en' ? 'Switch to 中文' : 'Switch to English'
              }
              className="text-xs font-medium w-auto px-3 cursor-pointer"
            >
              {label}
            </NavActionButton>

            <NavActionButton
              aria-label="GitHub repository"
              action={() =>
                window.open('https://github.com/noctisynth/merak', '_blank')
              }
              className="cursor-pointer"
            >
              <GithubIcon className="w-5 h-5" />
            </NavActionButton>
          </div>
        </div>
      </header>

      <section className="flex flex-col items-center justify-center text-center px-6 pt-32 pb-24">
        <div className="mb-8">
          <MerakLogo />
        </div>

        <h1 className="text-5xl md:text-6xl font-bold leading-tight max-w-4xl">
          {t('titlePrefix')}
          <span className="bg-linear-to-r from-indigo-600 to-blue-600 bg-clip-text text-transparent">
            {' '}
            {t('titleHighlight')}
          </span>{' '}
          {t('titleSuffix')}
        </h1>

        <p className="mt-6 text-lg text-gray-600 max-w-2xl">{t('desc')}</p>

        <div className="flex gap-4 mt-10">
          <Link
            to="/app"
            className="px-8 py-3 rounded-lg bg-indigo-600 text-white font-medium hover:bg-indigo-700 transition"
          >
            {t('enter')} →
          </Link>

          <a
            href="https://www.fumadocs.dev/"
            target="_blank"
            rel="noopener noreferrer"
            className="px-8 py-3 rounded-lg border border-gray-300 hover:bg-gray-100 transition"
          >
            {t('docs')}
          </a>
        </div>
      </section>
    </div>
  );
}
