export function createUiStore() {
  let sidebarCollapsed = $state(false);
  let showConnectionModal = $state(false);

  return {
    get sidebarCollapsed() { return sidebarCollapsed; },
    set sidebarCollapsed(v: boolean) { sidebarCollapsed = v; },
    get showConnectionModal() { return showConnectionModal; },
    set showConnectionModal(v: boolean) { showConnectionModal = v; },
  };
}

export const uiStore = createUiStore();
