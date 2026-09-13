<script lang="ts">
  import type { DetectedProject, UpdateCheck } from "../types";
  import AddSiteModal from "./modals/AddSiteModal.svelte";
  import DatabaseModal from "./modals/DatabaseModal.svelte";
  import UpdateModal from "./modals/UpdateModal.svelte";
  import SettingsModal from "./modals/SettingsModal.svelte";
  import Toast from "./Toast.svelte";

  let {
    toastMessage,
    showAddSiteModal,
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
