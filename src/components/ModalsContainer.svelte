<script lang="ts">
  import type { DetectedProject, SiteItem, UpdateCheck } from "../types";
  import AddSiteModal from "./modals/AddSiteModal.svelte";
  import CreateProjectModal, { type ScaffoldRequest } from "./modals/CreateProjectModal.svelte";
  import DatabaseModal from "./modals/DatabaseModal.svelte";
  import DeleteSiteModal from "./modals/DeleteSiteModal.svelte";
  import UpdateModal from "./modals/UpdateModal.svelte";
  import SettingsModal from "./modals/SettingsModal.svelte";
  import Toast from "./Toast.svelte";

  let {
    toastMessage,
    showAddSiteModal,
    showCreateProjectModal = false,
    showDeleteSiteModal = false,
    siteToDelete = null,
    newSitePath,
    newSiteDomain,
    newSiteType,
    newSiteTarget,
    domainSuffix,
    detectedProjectItem,
    showDatabaseModal,
    showUpdateModal,
    showSettingsModal,
    updateStatus,
    isCheckingUpdate,
    onCloseAddSite,
    onCloseCreateProject,
    onCloseDeleteSite,
    onConfirmDeleteSite,
    onRunInTerminal,
    onScaffoldGui,
    onPathChange,
    onDomainChange,
    onSuffixChange,
    onTypeChange,
    onTargetChange,
    onSubmitAddSite,
    onCloseDatabase,
    onOpenAdminer,
    onLaunchNative,
    onCreateDatabase,
    onCopyUrl,
    onCloseUpdate,
    onCheckUpdates,
    onSaveDomainSuffix,
    onCloseSettings,
  }: {
    toastMessage: string | null;
    showAddSiteModal: boolean;
    showCreateProjectModal?: boolean;
    showDeleteSiteModal?: boolean;
    siteToDelete?: SiteItem | null;
    newSitePath: string;
    newSiteDomain: string;
    newSiteType: string;
    newSiteTarget: string;
    domainSuffix: string;
    detectedProjectItem: DetectedProject | null;
    showDatabaseModal: boolean;
    showUpdateModal: boolean;
    showSettingsModal: boolean;
    updateStatus: UpdateCheck | null;
    isCheckingUpdate: boolean;
    onCloseAddSite: () => void;
    onCloseCreateProject?: () => void;
    onCloseDeleteSite?: () => void;
    onConfirmDeleteSite?: (deleteFiles: boolean) => void;
    onRunInTerminal?: (command: string, cwd?: string) => void;
    onScaffoldGui?: (req: ScaffoldRequest) => void;
    onPathChange: (p: string) => void;
    onDomainChange: (d: string) => void;
    onSuffixChange: (s: string) => void;
    onTypeChange: (t: string) => void;
    onTargetChange: (t: string) => void;
    onSubmitAddSite: () => void;
    onCloseDatabase: () => void;
    onOpenAdminer: () => void;
    onLaunchNative: () => void;
    onCreateDatabase: (name: string, engine?: string) => void;
    onCopyUrl: () => void;
    onCloseUpdate: () => void;
    onCheckUpdates: () => void;
    onSaveDomainSuffix: (s: string) => void;
    onCloseSettings: () => void;
  } = $props();
</script>

<Toast message={toastMessage} />

<CreateProjectModal
  show={showCreateProjectModal}
  {domainSuffix}
  onClose={() => onCloseCreateProject?.()}
  onRunInTerminal={(cmd, cwd) => onRunInTerminal?.(cmd, cwd)}
  onScaffoldGui={(req) => onScaffoldGui?.(req)}
/>

<AddSiteModal
  show={showAddSiteModal}
  path={newSitePath}
  domain={newSiteDomain}
  backendType={newSiteType}
  target={newSiteTarget}
  {domainSuffix}
  detectedProject={detectedProjectItem}
  onClose={onCloseAddSite}
  {onPathChange}
  {onDomainChange}
  {onSuffixChange}
  {onTypeChange}
  {onTargetChange}
  onSubmit={onSubmitAddSite}
/>

<DeleteSiteModal
  show={showDeleteSiteModal}
  site={siteToDelete}
  onClose={() => onCloseDeleteSite?.()}
  onConfirm={(del) => onConfirmDeleteSite?.(del)}
/>

<DatabaseModal
  show={showDatabaseModal}
  onClose={onCloseDatabase}
  {onOpenAdminer}
  {onLaunchNative}
  {onCreateDatabase}
  {onCopyUrl}
/>

<UpdateModal
  show={showUpdateModal}
  {updateStatus}
  {isCheckingUpdate}
  onClose={onCloseUpdate}
  {onCheckUpdates}
/>

<SettingsModal
  show={showSettingsModal}
  {domainSuffix}
  onSaveDomainSuffix={onSaveDomainSuffix}
  onClose={onCloseSettings}
/>
