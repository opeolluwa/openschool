export interface HomepageRouteItem {
  label: string;
  to: string;
  icon?: string;
  external?: boolean;
}

export interface FooterLinkSection {
  title: string;
  links: HomepageRouteItem[];
}

export const headerNavigation: HomepageRouteItem[] = [
  { label: "Features", to: "/#features", icon: "ri:stack-line" },
  { label: "Docs", to: "/public/docs", icon: "ri:book-open-line" },
  { label: "Community", to: "/public/community", icon: "ri:team-line" },
];

export const footerSections: FooterLinkSection[] = [
  {
    title: "Product",
    links: [
      { label: "Features", to: "/#features" },
      { label: "Self-Hosting", to: "/public/docs/self-hosting" },
      { label: "Demo", to: "/public/demo" },
      { label: "Changelog", to: "/public/changelog" },
    ],
  },
  {
    title: "Resources",
    links: [
      { label: "Documentation", to: "/public/docs" },
      { label: "API Reference", to: "/public/docs/api" },
      { label: "Tutorials", to: "/public/docs/tutorials" },
      { label: "Blog", to: "/public/blog" },
    ],
  },
  {
    title: "Community",
    links: [
      { label: "GitHub", to: "https://github.com/openschool", external: true },
      { label: "Discussions", to: "https://github.com/openschool/discussions", external: true },
      { label: "Contributing", to: "/public/docs/contributing" },
    ],
  },
  {
    title: "Legal",
    links: [
      { label: "Privacy Policy", to: "/public/privacy-policy" },
      { label: "Terms of Service", to: "/public/terms" },
      { label: "License", to: "/public/license" },
    ],
  },
];

export const socialLinks: HomepageRouteItem[] = [
  { label: "GitHub", to: "https://github.com/openschool", icon: "ri:github-line", external: true },
  { label: "Discord", to: "#", icon: "ri:discord-line", external: true },
  { label: "Twitter / X", to: "#", icon: "ri:twitter-x-fill", external: true },
  { label: "YouTube", to: "#", icon: "ri:youtube-line", external: true },
];

export function isExternalLink(item: HomepageRouteItem): boolean {
  return item.external === true || item.to.startsWith("http");
}
