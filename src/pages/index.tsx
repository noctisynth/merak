import { useTranslation } from 'react-i18next';
import { Link } from 'react-router';
import { useTheme } from '@/components/theme-provider';
import { Button } from '@/components/ui/button';
import { cn } from '@/lib/utils';
import { useLocale } from '../hooks/useLocale';
import { GithubIcon, MerakIcon, MerakLogo } from '../icon/icon';

interface NavActionButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {}

export function NavActionButton({ className, ...props }: NavActionButtonProps) {
  return (
    <Button
      variant="ghost"
      className={cn('rounded-lg h-9 w-9 cursor-pointer', className)}
      {...props}
    />
  );
}

export default function LandingPage() {
  const { t } = useTranslation('landing');
  const { lang, label, toggle } = useLocale();
  const { theme, setTheme } = useTheme();

  return (
    <div className="relative min-h-screen overflow-hidden bg-background text-foreground">
      <header className="fixed top-0 w-full z-50 backdrop-blur-xl bg-background/70 border-b border-border">
        <div className="max-w-7xl mx-auto px-6 h-16 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 flex items-center justify-center rounded-lg bg-linear-to-br from-primary to-accent shadow-sm">
              <MerakIcon className="w-5 h-5 text-primary-foreground" />
            </div>

            <span className="font-semibold text-lg tracking-tight">
              {t('brandName')}
            </span>
          </div>

          <div className="flex items-center gap-3">
            <NavActionButton
              aria-label="Toggle theme"
              onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
            >
              🌙
            </NavActionButton>

            <NavActionButton
              onClick={toggle}
              aria-label={
                lang === 'en' ? 'Switch to 中文' : 'Switch to English'
              }
            >
              {label}
            </NavActionButton>

            <NavActionButton
              aria-label="GitHub repository"
              onClick={() =>
                window.open('https://github.com/noctisynth/merak', '_blank')
              }
            >
              <GithubIcon className="w-5 h-5" />
            </NavActionButton>
          </div>
        </div>
      </header>

      <section className="flex flex-col items-center justify-center text-center px-6 pt-32 pb-24">
        <div className="text-primary mb-8">
          <MerakLogo />
        </div>

        <h1 className="text-5xl md:text-6xl font-bold leading-tight max-w-4xl">
          {t('titlePrefix')}
          <span className="bg-linear-to-r from-primary to-accent bg-clip-text text-transparent">
            {' '}
            {t('titleHighlight')}
          </span>{' '}
          {t('titleSuffix')}
        </h1>

        <p className="mt-6 text-lg text-muted-foreground max-w-2xl">
          {t('desc')}
        </p>

        <div className="flex gap-4 mt-10">
          <Link
            to="/app"
            className="px-8 py-3 rounded-lg bg-primary text-primary-foreground font-medium hover:bg-primary/90 transition"
          >
            {t('enter')} →
          </Link>

          <a
            href="https://www.fumadocs.dev/"
            target="_blank"
            rel="noopener noreferrer"
            className="px-8 py-3 rounded-lg border border-border hover:bg-muted transition"
          >
            {t('docs')}
          </a>
        </div>
      </section>
    </div>
  );
}
