import type { SVGProps } from 'react';

/**
 * All SVG icons should be added here and exported for reuse.
 * Size and color are controlled via `className` from the parent.
 */

// Merak Icon — used in navigation bars, buttons, and UI elements
export const MerakIcon = (props: SVGProps<SVGSVGElement>) => (
  <svg
    viewBox="0 0 128 128"
    fill="none"
    role="img"
    aria-label="Merak logo symbol"
    {...props}
  >
    <g stroke="currentColor" strokeWidth="10" strokeLinejoin="round">
      <polygon points="64,10 78,50 118,64 78,78 64,118 50,78 10,64 50,50" />
      <circle cx="64" cy="64" r="14" />
    </g>
  </svg>
);

// Merak Logo — used in branding contexts such as the landing page
export const MerakLogo = () => (
  <svg
    width="120"
    height="120"
    viewBox="0 0 128 128"
    role="img"
    aria-label="Merak logo"
  >
    <g
      stroke="currentColor"
      strokeWidth="5"
      strokeLinejoin="round"
      strokeLinecap="butt"
      vectorEffect="non-scaling-stroke"
      fill="none"
    >
      <polygon points="64,10 78,50 118,64 78,78 64,118 50,78 10,64 50,50" />
      <circle cx="64" cy="64" r="14" />
    </g>
  </svg>
);

// Github Icon
export const GithubIcon = (props: SVGProps<SVGSVGElement>) => (
  <svg
    viewBox="0 0 24 24"
    fill="currentColor"
    role="img"
    aria-label="GitHub repository"
    {...props}
  >
    <path d="M12 .5C5.73.5.5 5.73.5 12a11.5 11.5 0 008 10.94c.58.11.79-.25.79-.56v-2.02c-3.26.71-3.95-1.57-3.95-1.57-.53-1.34-1.3-1.7-1.3-1.7-1.07-.73.08-.72.08-.72 1.18.08 1.8 1.22 1.8 1.22 1.05 1.8 2.75 1.28 3.42.98.11-.76.41-1.28.74-1.57-2.6-.3-5.33-1.3-5.33-5.8 0-1.28.46-2.33 1.22-3.15-.12-.3-.53-1.53.12-3.18 0 0 1-.32 3.3 1.2a11.5 11.5 0 016 0c2.3-1.52 3.3-1.2 3.3-1.2.65 1.65.24 2.88.12 3.18.76.82 1.22 1.87 1.22 3.15 0 4.52-2.74 5.5-5.35 5.79.42.36.8 1.08.8 2.18v3.23c0 .31.21.68.8.56A11.5 11.5 0 0023.5 12C23.5 5.73 18.27.5 12 .5z" />
  </svg>
);
