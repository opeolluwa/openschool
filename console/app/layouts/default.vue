<script setup lang="ts">
// import useLogout from "~/composables/useLogout";
import routes  from "~/data/routes";
// import { emailsUnreadCountQuery, readCount } from "~/queries/emails";
// import { notificationsUnreadCountQuery } from "~/queries/notifications";
// import { useUserInformationStore } from "~/stores/users";
import RyderShell, {
  type ShellUser as RyderUser,
} from "@opeolluwa/ryder/layouts/RyderShell.vue";
// import { usePermissions } from "~/composables/usePermissions";

const route = useRoute();
const isFullHeight = computed(() => route.meta.fullHeight === true);

// const userStore = useUserInformationStore();
// const logout = useLogout();
// const { can, ready } = usePermissions();

// const user = computed<RyderUser>(() => ({
//   name: userStore.user.firstName ?? "",
//   avatar: { src: userStore.user.profilePicture || undefined },
// }));

// const notificationsUnread = useQuery(notificationsUnreadCountQuery);
// const notificationsUnreadCount = computed(
//   () => notificationsUnread.data.value?.count ?? 0,
// );

// const emailsUnread = useQuery(emailsUnreadCountQuery);
// const emailsUnreadCount = computed(() => readCount(emailsUnread.data.value));

// const navItems = computed(() => {
//   if (!ready.value) return [];
// 
//   const filtered = routes.filter((item) => canSeeRoute(item, can));
// 
//   return filtered.map((item) =>
//     item.to === "/emails" && emailsUnreadCount.value > 0
//       ? { ...item, badge: String(emailsUnreadCount.value) }
//       : item.to === "/notifications" && notificationsUnreadCount.value > 0
//         ? { ...item, badge: String(notificationsUnreadCount.value) }
//         : item,
//   );
// });
</script>

<template>
  <RyderShell


:nav-items="routes"
    root-path="/"
    :fixed-height="isFullHeight"
    :content-class="isFullHeight ? '!p-0 h-[calc(100dvh-4rem)]' : ''"

  >
    <template #sidebar-brand>
      <AppLogo />
    </template>

    <template #mobile-nav>
      <RyderSideNav
        :items="navItems"
        :user="user"
        root-path="/"
        @logout="logout"
      />
    </template>

    <slot />
  </RyderShell>
</template>
