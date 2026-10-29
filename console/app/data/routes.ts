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
  { label: "Home", to: "/", icon: "ri:home-line" },
  { label: "Library", to: "/library", icon: "ri:book-open-line" },
  { label: "Programs", to: "/programs", icon: "ri:graduation-cap-line" },
  { label: "Student", to: "/student", icon: "ri:user-line" },
  { label: "Staff", to: "/staff", icon: "ri:team-line" },
  { label: "Parent", to: "/parent", icon: "ri:parent-line" },
  { label: "Blog", to: "/public/blog", icon: "ri:article-line" },
];

export const footerSections: FooterLinkSection[] = [
  {
    title: "Academics",
    links: [
      { label: "Programs", to: "/programs" },
      { label: "Library", to: "/library" },
      { label: "Admissions", to: "/admission" },
      { label: "Blog", to: "/public/blog" },
    ],
  },
  {
    title: "Campus",
    links: [
      { label: "Student Portal", to: "/student" },
      { label: "Staff Portal", to: "/staff" },
      { label: "Parent Portal", to: "/parent" },
    ],
  },
  {
    title: "Resources",
    links: [
      { label: "Documentation", to: "/public/docs" },
      { label: "Tutorials", to: "/public/docs/tutorials" },
      { label: "Community", to: "/public/community" },
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
  { label: "GitHub", to: "https://github.com/opeolluwa/openschool", icon: "ri:github-line", external: true },
  { label: "Discord", to: "#", icon: "ri:discord-line", external: true },
  { label: "Twitter / X", to: "#", icon: "ri:twitter-x-fill", external: true },
  { label: "YouTube", to: "#", icon: "ri:youtube-line", external: true },
];

export function isExternalLink(item: HomepageRouteItem): boolean {
  return item.external === true || item.to.startsWith("http");
}
