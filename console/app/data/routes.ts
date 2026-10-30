export type RouteItem = {
  label: string;
  to: string;
  icon?: string;
  activeIcon?: string;
  external?: boolean;
  resource?: string;
  action?: string;
  anyOf?: string[];
};


const routes: RouteItem[] = [
  // Overview
  {
    label: "Dashboard",
    to: "/",
    icon: "ri:dashboard-line",
    activeIcon: "ri:dashboard-fill",
    resource: "metrics",
    action: "read",
  },

  // Academic Management
  {
    label: "Academics",
    to: "/academics",
    icon: "ri:graduation-cap-line",
    resource: "academics",
    action: "read",
  },
  {
    label: "Programs",
    to: "/programs",
    icon: "ri:book-open-line",
    resource: "programs",
    action: "read",
  },
  {
    label: "Courses",
    to: "/courses",
    icon: "ri:book-2-line",
    resource: "courses",
    action: "read",
  },
  {
    label: "Departments",
    to: "/departments",
    icon: "ri:building-2-line",
    resource: "departments",
    action: "read",
  },
  {
    label: "Classes",
    to: "/classes",
    icon: "ri:school-line",
    resource: "classes",
    action: "read",
  },
  {
    label: "Timetable",
    to: "/timetable",
    icon: "ri:calendar-schedule-line",
    resource: "timetable",
    action: "read",
  },
  {
    label: "Academic Calendar",
    to: "/academic-calendar",
    icon: "ri:calendar-event-line",
    resource: "academic_calendar",
    action: "read",
  },

  // People Management
  {
    label: "Students",
    to: "/students",
    icon: "ri:graduation-cap-line",
    resource: "students",
    action: "read",
  },
  {
    label: "Staff",
    to: "/staff",
    icon: "ri:team-line",
    resource: "staff",
    action: "read",
  },
  {
    label: "Parents & Guardians",
    to: "/guardians",
    icon: "ri:parent-line",
    resource: "guardians",
    action: "read",
  },
  {
    label: "Admissions",
    to: "/admissions",
    icon: "ri:user-add-line",
    resource: "admissions",
    action: "read",
  },

  // Learning & Assessment
  {
    label: "Attendance",
    to: "/attendance",
    icon: "ri:user-follow-line",
    resource: "attendance",
    action: "read",
  },
  {
    label: "Examinations",
    to: "/examinations",
    icon: "ri:file-list-3-line",
    resource: "examinations",
    action: "read",
  },
  {
    label: "Results & Grades",
    to: "/results",
    icon: "ri:bar-chart-box-line",
    resource: "results",
    action: "read",
  },
  {
    label: "Assignments",
    to: "/assignments",
    icon: "ri:task-line",
    resource: "assignments",
    action: "read",
  },

  // Finance
  {
    label: "Finance",
    to: "/finance",
    icon: "ri:wallet-3-line",
    resource: "finance",
    action: "read",
  },
  {
    label: "Payments",
    to: "/payments",
    icon: "ri:bank-card-line",
    resource: "payments",
    action: "read",
  },
  {
    label: "Invoices",
    to: "/invoices",
    icon: "ri:bill-line",
    resource: "invoices",
    action: "read",
  },

  // Resources
  {
    label: "Library",
    to: "/library",
    icon: "ri:book-shelf-line",
    resource: "library",
    action: "read",
  },
  {
    label: "Inventory",
    to: "/inventory",
    icon: "ri:archive-line",
    resource: "inventory",
    action: "read",
  },

  // Communication
  {
    label: "Announcements",
    to: "/announcements",
    icon: "ri:megaphone-line",
    resource: "announcements",
    action: "read",
  },
  {
    label: "Messages",
    to: "/messages",
    icon: "ri:message-3-line",
    resource: "messages",
    action: "read",
  },

  // Administration
  {
    label: "People & Access",
    to: "/people",
    icon: "ri:shield-user-line",
    anyOf: [
      { resource: "users", action: "read" },
      { resource: "roles", action: "read" },
      { resource: "permissions", action: "read" },
    ],
  },
  {
    label: "Reports",
    to: "/reports",
    icon: "ri:pie-chart-line",
    resource: "reports",
    action: "read",
  },
  {
    label: "Settings",
    to: "/settings",
    icon: "ri:settings-3-line",
    resource: "settings",
    action: "read",
  },
];

export default routes;