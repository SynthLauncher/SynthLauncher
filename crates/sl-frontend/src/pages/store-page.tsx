"use client"

import { useEffect, useState } from "react"
import { CurseforgeStoreCard, ModrinthStoreCard } from "@/components/layout/pages/store/store-card"
import { Input } from "@/components/ui/input"
import {
  SearchIcon,
  WifiOff,
  Loader2,
  AlertCircle,
  ChevronFirst,
  ChevronLeft,
  ChevronRight,
  ChevronLast,
  Globe,
  X,
  Download,
  Blocks,
  Package,
  ChevronDown,
} from "lucide-react"
import { StoreCategorySelector } from "@/components/layout/pages/store/store-category-selector"
import {
  getCurseforgeStoreSearch,
  getModrinthProjectVersions,
  getModrinthStoreSearch,
  installModrinthProject,
} from "@/lib/commands/store"
import { getInstances } from "@/lib/commands/instances"
import type { CurseforgeProject, CurseforgeSearchResult } from "@/lib/types/store/curseforge"
import type { ModrinthProjectVersion, ModrinthSearchHit, ModrinthSearchResult } from "@/lib/types/store/modrinth"
import type { Instance } from "@/lib/types/instances"
import { Pagination, PaginationContent, PaginationItem, PaginationLink } from "@/components/ui/pagination"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { ToastError, ToastSuccess } from "@/components/toasters"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"

type StoreType = "modrinth" | "curseforge"
type StoreErrorType = "failed" | "offline" | null
type ContentCategoryType = "modpack" | "mod" | "resourcepack" | "shader"

interface StoreState {
  store: StoreType
  modrinthResult?: ModrinthSearchResult
  curseforgeResult?: CurseforgeSearchResult
  searchQuery: string
  category: ContentCategoryType
  loading: boolean
  error: StoreErrorType
  isOnline: boolean
  page: number
  selectedInstance?: Instance
}

interface ContentState {
  selectedContent: ModrinthSearchHit | CurseforgeProject | null
  selectedVersion?: ModrinthProjectVersion
  instances: Instance[]
  contentVersions: ModrinthProjectVersion[]
  isVersionDialogOpen: boolean
}

const CURSEFORGE_CLASS_IDS: Record<string, number> = {
  modpack: 4471,
  mod: 6,
  resourcepack: 12,
  shader: 6552,
}

const StoreFilter = ({
  store,
  category,
  searchQuery,
  selectedInstance,
  instances,
  totalResults,
  loading,
  error,
  onStoreChange,
  onCategoryChange,
  onSearchChange,
  onInstanceSelect,
}: {
  store: StoreType
  category: ContentCategoryType
  searchQuery: string
  selectedInstance?: Instance
  instances: Instance[]
  totalResults?: number
  loading: boolean
  error: StoreErrorType
  onStoreChange: (store: StoreType) => void
  onCategoryChange: (category: ContentCategoryType) => void
  onSearchChange: (query: string) => void
  onInstanceSelect: (instance: Instance) => void
}) => {
  return (
    <div className="flex flex-col gap-4">
      <div className="flex flex-col lg:flex-row items-start lg:items-center gap-4 lg:gap-6">
        <div className="flex flex-col sm:flex-row items-start sm:items-center gap-4 flex-1">
          <div className="flex items-center gap-4">
            <StoreCategorySelector
              values={["modrinth", "curseforge"]}
              displayValues={["Modrinth", "Curseforge"]}
              defaultValue="modrinth"
              onValueChange={(v) => onStoreChange(v as StoreType)}
            />
            <div className="w-[2px] h-6 bg-neutral-600" />
            <StoreCategorySelector
              values={["modpack", "mod", "shader", "resourcepack"]}
              displayValues={["Modpacks", "Mods", "Shaders", "Resource Packs"]}
              defaultValue="modpack"
              onValueChange={(v) => onCategoryChange(v as ContentCategoryType)}
            />
          </div>

          {category !== "modpack" && (
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <button
                  type="button"
                  className="flex items-center gap-3 px-4 py-2 bg-[#2b3136] hover:bg-[#323842] rounded-lg border border-[#3a3d4f] text-white transition-all duration-200 min-w-[200px] justify-between"
                >
                  <div className="flex items-center gap-3">
                    <div className="w-8 h-8 bg-gradient-to-br from-[#424555] to-[#363a4a] rounded-lg flex items-center justify-center">
                      <Blocks className="w-4 h-4 text-blue-400" />
                    </div>
                    <div className="text-left">
                      <div className="text-sm font-medium">
                        {selectedInstance ? selectedInstance.name : "Select Instance"}
                      </div>
                      {selectedInstance && (
                        <div className="text-xs text-neutral-400">
                          {selectedInstance.mod_loader} • {selectedInstance.mc_version}
                        </div>
                      )}
                    </div>
                  </div>
                  <ChevronDown className="w-4 h-4 transition-transform duration-200" />
                </button>
              </DropdownMenuTrigger>
              <DropdownMenuContent
                className="w-[300px] bg-[#2b3136] border-[#3a3d4f] shadow-xl max-h-64 overflow-y-auto"
                align="start"
              >
                {instances.length > 0 ? (
                  instances.map((instance) => (
                    <DropdownMenuItem
                      key={instance.name}
                      onClick={() => onInstanceSelect(instance)}
                      className="px-4 py-3 text-left hover:bg-[#323842] focus:bg-[#323842] transition-all duration-150 cursor-pointer"
                    >
                      <div className="flex items-center gap-3">
                        <div className="w-8 h-8 bg-gradient-to-br from-[#424555] to-[#363a4a] rounded-lg flex items-center justify-center">
                          <Blocks className="w-4 h-4 text-blue-400" />
                        </div>
                        <div>
                          <div className="text-sm font-medium text-white">{instance.name}</div>
                          <div className="text-xs text-neutral-400">
                            {instance.mod_loader} • {instance.mc_version}
                          </div>
                        </div>
                      </div>
                    </DropdownMenuItem>
                  ))
                ) : (
                  <div className="px-4 py-6 text-center">
                    <div className="text-neutral-400 text-sm mb-2">No instances found</div>
                    <Button className="bg-blue-600 hover:bg-blue-500 text-white text-xs px-3 py-1">
                      Create Instance
                    </Button>
                  </div>
                )}
              </DropdownMenuContent>
            </DropdownMenu>
          )}
        </div>

        {!loading && !error && totalResults && (
          <div className="flex items-center gap-2 text-sm text-neutral-400">
            <Globe className="w-4 h-4" />
            <span>
              <span className="text-white font-medium">{totalResults.toLocaleString()}</span> results from{" "}
              <span className="text-blue-400 font-medium capitalize">{store}</span>
            </span>
          </div>
        )}
      </div>

      <div className="relative w-full">
        <Input
          icon={<SearchIcon className="w-4 h-4 text-neutral-400" />}
          placeholder={`Search ${category}s...`}
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
          className="bg-neutral-700/50 border-neutral-700 text-white placeholder-neutral-400 focus:border-sky-400/50 focus:bg-neutral-800/70 transition-all duration-300 h-12 text-base"
        />
        {searchQuery && (
          <div className="absolute right-3 top-1/2 -translate-y-1/2">
            <button
              onClick={() => onSearchChange("")}
              className="w-6 h-6 rounded-full bg-neutral-600 hover:bg-neutral-500 flex items-center justify-center text-neutral-300 hover:text-white transition-colors"
            >
              <X width={15} height={15} />
            </button>
          </div>
        )}
      </div>

      {category !== "modpack" && !selectedInstance && (
        <div className="bg-yellow-500/10 border border-yellow-500/30 rounded-lg p-4">
          <div className="flex items-center gap-3">
            <div className="w-8 h-8 bg-yellow-500/20 rounded-lg flex items-center justify-center">
              <Blocks className="w-4 h-4 text-yellow-400" />
            </div>
            <div>
              <h3 className="text-yellow-400 font-medium text-sm">Select an Instance</h3>
              <p className="text-yellow-300/80 text-sm">Choose an instance above to install {category}s to it.</p>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}

const LoadingState = ({ store, category }: { store: StoreType; category: ContentCategoryType }) => (
  <div className="flex flex-col items-center justify-center py-16 px-6">
    <div className="relative mb-4">
      <Loader2 className="animate-spin w-8 h-8 text-blue-400" />
      <div className="absolute inset-0 w-8 h-8 border-2 border-blue-400/20 rounded-full animate-pulse"></div>
    </div>
    <h3 className="text-lg font-semibold text-white mb-2">Loading Content</h3>
    <p className="text-neutral-400 text-center">
      Searching {store} for the best {category}s...
    </p>
  </div>
)

const ErrorState = ({ error, store }: { error: StoreErrorType; store: StoreType }) => {
  if (error == "offline") {
    return (
      <div className="flex flex-col items-center justify-center py-16 px-6">
        <div className="w-20 h-20 bg-neutral-700/50 rounded-full flex items-center justify-center mb-6">
          <WifiOff className="w-10 h-10 text-neutral-400" />
        </div>
        <h3 className="text-xl font-semibold text-white mb-3">You're Offline</h3>
        <p className="text-neutral-400 text-center max-w-md mb-4">
          Please check your internet connection to browse and download content from the store.
        </p>
        <div className="bg-neutral-700/50 rounded-lg p-4 border border-neutral-600">
          <p className="text-sm text-neutral-400">💡 Tip: You can still manage your existing instances while offline</p>
        </div>
      </div>
    )
  }

  if (error == "failed") {
    return (
      <div className="flex flex-col items-center justify-center py-16 px-6">
        <div className="w-20 h-20 bg-red-500/10 rounded-full flex items-center justify-center mb-6">
          <AlertCircle className="w-10 h-10 text-red-400" />
        </div>
        <h3 className="text-xl font-semibold text-red-400 mb-3">Failed to Load Content</h3>
        <p className="text-neutral-400 text-center max-w-md mb-4">
          We couldn't connect to the {store} servers. Please try again later.
        </p>
      </div>
    )
  }

  return null
}

const StoreContent = ({
  store,
  category,
  selectedInstance,
  modrinthResult,
  curseforgeResult,
  onContentClick,
}: {
  store: StoreType
  category: ContentCategoryType
  selectedInstance?: Instance
  modrinthResult?: ModrinthSearchResult
  curseforgeResult?: CurseforgeSearchResult
  onContentClick: (content: ModrinthSearchHit | CurseforgeProject) => void
}) => {
  const handleClick = (content: ModrinthSearchHit | CurseforgeProject) => {
    if (category !== "modpack" && !selectedInstance) {
      ToastError("Please select an instance first")
      return
    }
    onContentClick(content)
  }

  return (
    <div className="grid grid-cols-1 gap-4">
      {store == "curseforge" &&
        curseforgeResult?.data.map((hit) => (
          <div key={hit.id} onClick={() => handleClick(hit)} className="cursor-pointer">
            <CurseforgeStoreCard hit={hit} />
          </div>
        ))}

      {store == "modrinth" &&
        modrinthResult?.hits.map((hit) => (
          <div key={hit.project_id} onClick={() => handleClick(hit)} className="cursor-pointer">
            <ModrinthStoreCard hit={hit} />
          </div>
        ))}
    </div>
  )
}

const StorePagination = ({
  page,
  lastPage,
  onPageChange,
}: {
  page: number
  lastPage: number | null
  onPageChange: (page: number) => void
}) => {
  if (!lastPage || lastPage < 1) return null

  return (
    <div className="border-t border-[#3a3d4f]/50 p-6">
      <Pagination>
        <PaginationContent className="gap-2">
          {page > 1 && (
            <>
              <PaginationItem>
                <PaginationLink
                  onClick={() => onPageChange(1)}
                  aria-label="Go to first page"
                  size="icon"
                  className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                >
                  <ChevronFirst className="h-4 w-4" />
                </PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink
                  onClick={() => onPageChange(page - 1)}
                  aria-label="Go to previous page"
                  size="icon"
                  className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                >
                  <ChevronLeft className="h-4 w-4" />
                </PaginationLink>
              </PaginationItem>
            </>
          )}
          <PaginationItem>
            <PaginationLink
              size="icon"
              onClick={() => onPageChange(page)}
              isActive
              className="bg-sky-600 border-sky-500 text-white hover:bg-sky-500"
            >
              {page}
            </PaginationLink>
          </PaginationItem>
          {lastPage && page + 1 <= lastPage && (
            <PaginationItem>
              <PaginationLink
                size="icon"
                onClick={() => onPageChange(page + 1)}
                className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
              >
                {page + 1}
              </PaginationLink>
            </PaginationItem>
          )}
          {lastPage && page + 2 <= lastPage && (
            <PaginationItem>
              <PaginationLink
                size="icon"
                onClick={() => onPageChange(page + 2)}
                className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
              >
                {page + 2}
              </PaginationLink>
            </PaginationItem>
          )}
          {lastPage && page < lastPage && (
            <>
              <PaginationItem>
                <PaginationLink
                  onClick={() => onPageChange(page + 1)}
                  aria-label="Go to next page"
                  size="icon"
                  className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                >
                  <ChevronRight className="h-4 w-4" />
                </PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink
                  onClick={() => onPageChange(lastPage)}
                  aria-label="Go to last page"
                  size="icon"
                  className="bg-[#424555]/30 border-[#4a4f63]/50 text-neutral-300 hover:bg-[#424555]/50 hover:text-white hover:border-blue-400/50 transition-all duration-300"
                >
                  <ChevronLast className="h-4 w-4" />
                </PaginationLink>
              </PaginationItem>
            </>
          )}
        </PaginationContent>
      </Pagination>
      <div className="flex items-center justify-center mt-4 text-sm text-neutral-400">
        <span>
          Page <span className="text-white font-medium">{page}</span> of{" "}
          <span className="text-white font-medium">{lastPage}</span>
        </span>
      </div>
    </div>
  )
}

const VersionDialog = ({
  isOpen,
  onClose,
  selectedContent,
  selectedInstance,
  category,
  contentVersions,
  onVersionSelect,
}: {
  isOpen: boolean
  onClose: () => void
  selectedContent: any
  selectedInstance?: Instance
  category: string
  contentVersions?: ModrinthProjectVersion[]
  onVersionSelect: (version: ModrinthProjectVersion) => void
}) => (
  <Dialog open={isOpen} onOpenChange={onClose}>
    <DialogContent className="bg-neutral-800 border-neutral-700 text-white max-w-4xl max-h-[80vh] overflow-hidden">
      <DialogHeader className="border-b border-neutral-700 pb-4">
        <DialogTitle className="text-xl font-bold text-white flex items-center gap-3">
          <Package className="w-5 h-5 text-sky-400" />
          Choose Version - {selectedContent?.title || selectedContent?.name}
        </DialogTitle>
        <DialogDescription className="text-neutral-400">
          Installing to: <span className="text-white font-medium">{selectedInstance?.name}</span> • Select which version
          of this {category} you want to install
        </DialogDescription>
      </DialogHeader>

      <div className="py-4">
        <div className="space-y-3 max-h-96 overflow-y-auto">
          {contentVersions && contentVersions.length > 0 ? (
            contentVersions.map((version) => (
              <div
                key={version.id}
                className="bg-[#1D2026] hover:bg-[#1D2026]/90 rounded-lg p-5 flex gap-4 max-w-full transition-colors duration-200 border border-transparent hover:border-neutral-700/50 cursor-pointer group"
                onClick={() => onVersionSelect(version)}
              >
                <div className="flex flex-col grow justify-between min-w-0">
                  <h1 className="text-white text-lg font-bold leading-tight group-hover:text-sky-400 transition-colors">
                    {version.name}
                  </h1>
                  <div className="flex gap-2 mt-3 flex-wrap">
                    {version.game_versions.map((gameVersion) => (
                      <span
                        key={gameVersion}
                        className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-500/20 text-blue-400 border border-blue-500/30"
                      >
                        {gameVersion}
                      </span>
                    ))}
                    {version.loaders.map((loader) => (
                      <span
                        key={loader}
                        className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-emerald-500/20 text-emerald-400 border border-emerald-500/30"
                      >
                        {loader}
                      </span>
                    ))}
                  </div>
                </div>
                <div className="flex flex-col justify-between items-end ml-4 flex-shrink-0">
                  <Button
                    variant="install"
                    className="gap-2 hover:scale-105 transition-transform duration-200"
                    onClick={() => onVersionSelect(version)}
                  >
                    <Download className="w-4 h-4" />
                    Install
                  </Button>
                </div>
              </div>
            ))
          ) : (
            <div className="flex flex-col items-center justify-center py-12">
              <div className="w-16 h-16 bg-neutral-700/50 rounded-full flex items-center justify-center mb-4">
                <Package className="w-8 h-8 text-neutral-400" />
              </div>
              <h3 className="text-lg font-semibold text-white mb-2">No Compatible Versions</h3>
              <p className="text-neutral-400 text-center max-w-md">
                No versions of this {category} are compatible with your selected instance.
              </p>
            </div>
          )}
        </div>
      </div>

      <div className="flex justify-end gap-3 pt-4 border-t border-neutral-700">
        <Button
          variant="outline"
          onClick={onClose}
          className="border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white"
        >
          Cancel
        </Button>
      </div>
    </DialogContent>
  </Dialog>
)

export const StorePage = () => {
  const [storeState, setStoreState] = useState<StoreState>({
    store: "modrinth",
    searchQuery: "",
    category: "modpack",
    loading: false,
    error: null,
    isOnline: true,
    page: 1,
    selectedInstance: undefined,
  })

  const [contentState, setContentState] = useState<ContentState>({
    selectedContent: null,
    selectedVersion: undefined,
    instances: [],
    contentVersions: [],
    isVersionDialogOpen: false,
  })

  useEffect(() => {
    const fetchInstances = async () => {
      const instanceList = await getInstances()
      setContentState((prev) => ({ ...prev, instances: instanceList || [] }))
    }
    fetchInstances()
  }, [])

  useEffect(() => {
    const checkOnline = () => setStoreState((prev) => ({ ...prev, isOnline: navigator.onLine }))
    window.addEventListener("online", checkOnline)
    window.addEventListener("offline", checkOnline)
    checkOnline()
    return () => {
      window.removeEventListener("online", checkOnline)
      window.removeEventListener("offline", checkOnline)
    }
  }, [])

  useEffect(() => {
    const fetchStoreData = async () => {
      if (!storeState.isOnline) {
        setStoreState((prev) => ({ ...prev, error: "offline" }))
        return
      }

      setStoreState((prev) => ({ ...prev, loading: true, error: null }))

      try {
        if (storeState.store == "curseforge") {
          const classId = CURSEFORGE_CLASS_IDS[storeState.category] ?? 4471
          const result = await getCurseforgeStoreSearch(storeState.searchQuery, classId, storeState.page)
          setStoreState((prev) => ({ ...prev, curseforgeResult: result }))
        } else {
          const result = await getModrinthStoreSearch(storeState.searchQuery, storeState.category, storeState.page)
          setStoreState((prev) => ({ ...prev, modrinthResult: result }))
        }
      } catch (err) {
        setStoreState((prev) => ({ ...prev, error: "failed" }))
      } finally {
        setStoreState((prev) => ({ ...prev, loading: false }))
      }
    }

    fetchStoreData()
  }, [storeState.searchQuery, storeState.category, storeState.isOnline, storeState.page, storeState.store])

  const handleStoreChange = (store: StoreType) => {
    setStoreState((prev) => ({ ...prev, store, page: 1 }))
  }

  const handleCategoryChange = (category: ContentCategoryType) => {
    setStoreState((prev) => ({ ...prev, category: category, page: 1 }))
  }

  const handleSearchChange = (searchQuery: string) => {
    setStoreState((prev) => ({ ...prev, searchQuery, page: 1 }))
  }

  const handlePageChange = (page: number) => {
    setStoreState((prev) => ({ ...prev, page }))
  }

  const handleInstanceSelect = (instance: Instance) => {
    setStoreState((prev) => ({ ...prev, selectedInstance: instance }))
  }

  const handleContentClick = async (content: ModrinthSearchHit | CurseforgeProject) => {
    if (storeState.category === "modpack") {
      return
    }

    if (!storeState.selectedInstance) {
      ToastError("Please select an instance first")
      return
    }

    setContentState((prev) => ({
      ...prev,
      selectedContent: content,
    }))

    try {
      const contentSlug = content.slug || (content as any).id
      const versions = await getModrinthProjectVersions(
        contentSlug as string,
        storeState.selectedInstance.mc_version,
        storeState.selectedInstance.mod_loader.toString(),
        storeState.category
      )

      setContentState((prev) => ({
        ...prev,
        contentVersions: versions as ModrinthProjectVersion[],
        isVersionDialogOpen: true,
      }))
    } catch (error) {
      ToastError(`Failed to fetch versions: ${error}`)
    }
  }

  let installed = false;
  const handleVersionSelect = async (version: ModrinthProjectVersion) => {
    if (!storeState.selectedInstance || !contentState.selectedContent || installed) return;
    installed = true;

    try {
      await installModrinthProject(
        contentState.selectedContent.slug as string,
        version.id,
        storeState.selectedInstance.name,
        storeState.category
      )
      ToastSuccess(`Successfully installed ${contentState.selectedContent.slug}`)
    } catch (error) {
      ToastError(`Installation failed: ${error}`)
    } finally {
      installed = false
    }

    setContentState((prev) => ({
      ...prev,
      selectedContent: null,
      selectedVersion: undefined,
      contentVersions: [],
      isVersionDialogOpen: false,
    }))
  }


  const handleCloseVersionDialog = () => {
    setContentState((prev) => ({
      ...prev,
      selectedContent: null,
      selectedVersion: undefined,
      contentVersions: [],
      isVersionDialogOpen: false,
    }))
  }

  const lastPage =
    storeState.store == "modrinth"
      ? storeState.modrinthResult?.total_hits
        ? Math.ceil(storeState.modrinthResult.total_hits / 16)
        : null
      : storeState.curseforgeResult?.pagination.totalCount
        ? 625
        : null

  const totalResults =
    storeState.store == "modrinth"
      ? storeState.modrinthResult?.total_hits
      : storeState.curseforgeResult?.pagination?.totalCount

  return (
    <div className="flex flex-col gap-4">
      <StoreFilter
        store={storeState.store}
        category={storeState.category}
        searchQuery={storeState.searchQuery}
        selectedInstance={storeState.selectedInstance}
        instances={contentState.instances}
        totalResults={totalResults}
        loading={storeState.loading}
        error={storeState.error}
        onStoreChange={handleStoreChange}
        onCategoryChange={handleCategoryChange}
        onSearchChange={handleSearchChange}
        onInstanceSelect={handleInstanceSelect}
      />

      {storeState.loading && <LoadingState store={storeState.store} category={storeState.category} />}

      {storeState.error && <ErrorState error={storeState.error} store={storeState.store} />}

      {!storeState.loading && !storeState.error && (
        <StoreContent
          store={storeState.store}
          category={storeState.category}
          selectedInstance={storeState.selectedInstance}
          modrinthResult={storeState.modrinthResult}
          curseforgeResult={storeState.curseforgeResult}
          onContentClick={handleContentClick}
        />
      )}

      {!storeState.loading && !storeState.error && (
        <StorePagination page={storeState.page} lastPage={lastPage} onPageChange={handlePageChange} />
      )}

      <VersionDialog
        isOpen={contentState.isVersionDialogOpen}
        onClose={handleCloseVersionDialog}
        selectedContent={contentState.selectedContent}
        selectedInstance={storeState.selectedInstance}
        category={storeState.category}
        contentVersions={contentState.contentVersions}
        onVersionSelect={handleVersionSelect}
      />
    </div>
  )
}
