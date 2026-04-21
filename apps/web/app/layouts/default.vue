<template>
    <div class="dashboard-wrapper">
        <aside :class="['sidebar', { 'sidebar-hidden': !isSidebarOpen }]">
            <div class="sidebar-header">
                <h2>Dashboard</h2>
            </div>
            <Menu :model="navigationItems">
                <template #itemicon="{ item }">
                    <i :class="item.icon"></i>
                </template>
                <template #item="{ item }">
                    <NuxtLink :to="item.to" class="menu-link">
                        <span>{{ item.label }}</span>
                    </NuxtLink>
                </template>
            </Menu>
        </aside>

        <main class="content">
            <header class="topbar">
                <Button icon="pi pi-bars" @click="toggleSidebar" text />
                <Avatar size="large" image="/avatar.jpg" shape="circle" />
            </header>

            <section class="content-area">
                <slot />
            </section>
        </main>
    </div>
</template>

<script setup>
import { ref } from 'vue';

const isSidebarOpen = ref(true);

const navigationItems = [
    { label: 'Dashboard', icon: 'pi pi-home', to: '/about' },
    { label: 'Users', icon: 'pi pi-chart-line', to: '/users' },
    { label: 'Settings', icon: 'pi pi-cog', to: '/settings' }
];

const toggleSidebar = () => {
    isSidebarOpen.value = !isSidebarOpen.value;
};
</script>

<style scoped>
div{
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    color: black;
}
.dashboard-wrapper {
    display: flex;
    min-height: 100vh;
    background-color: #f5f5f5;
}

.sidebar {
    width: 280px;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    border-right: 1px solid #ddd;
    box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
    transition: transform 0.3s ease, width 0.3s ease;
    overflow-y: auto;
}

.sidebar-hidden {
    transform: translateX(-100%);
    width: 0;
}

.sidebar-header {
    padding: 2rem 1.5rem;
    color: white;
    border-bottom: 2px solid rgba(255, 255, 255, 0.2);
}

.sidebar-header h2 {
    margin: 0;
    font-size: 1.5rem;
    font-weight: 600;
}

.content {
    flex: 1;
    display: flex;
    flex-direction: column;
    background-color: #ffffff;
}

.topbar {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 1.5rem 2rem;
    background: linear-gradient(90deg, #667eea 0%, #764ba2 100%);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    color: white;
}

.content-area {
    padding: 2rem;
    flex: 1;
    background-color: #f9f9f9;
}

:deep(.p-menu) {
    background-color: transparent;
    border: none;
}

:deep(.p-menu .p-menuitem-link) {
    color: rgba(255, 255, 255, 0.8);
    border-radius: 8px;
    margin: 0.5rem 0.75rem;
    padding: 0.75rem 1rem;
    transition: all 0.3s ease;
}

:deep(.p-menu .p-menuitem-link:hover) {
    background-color: rgba(255, 255, 255, 0.15);
    color: white;
}

:deep(.p-menu .p-menuitem-link.router-link-active) {
    background-color: rgba(255, 255, 255, 0.25);
    color: white;
    font-weight: 600;
}

.menu-link {
    display: flex;
    padding: 1rem;
    align-items: center;
    text-decoration: none;
    color: inherit;
    width: 100%;
}

.menu-link i {
    font-size: 1.2rem;
}

@media (max-width: 768px) {
    .sidebar {
        position: absolute;
        height: 100vh;
        z-index: 100;
    }

    .sidebar-hidden {
        transform: translateX(-100%);
    }

    .sidebar-status {
        display: none;
    }
}
</style>
