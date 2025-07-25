import { getGameInfo, getInstances, killInstance, launchInstance } from "@/lib/commands/instances"
import { openInstanceFolder } from "@/lib/commands/launcher"
import type { GameInfo, Instance } from "@/lib/types/instances"
import {
  Blocks,
  Ellipsis,
  FolderUp,
  Loader2,
  Download,
  FileText,
  Save,
  ImageIcon,
  Terminal,
  Calendar,
  Clock,
  Gamepad2,
  AlertCircle,
  Folder,
  Play,
  Check,
  Copy,
} from "lucide-react"
import { useEffect, useState } from "react"
import { useParams } from "react-router-dom"
import { ToastInfo, ToastSuccess } from "@/components/toasters"
import { useTranslation } from "react-i18next"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { Button } from "@/components/ui/button"
import { listen } from "@tauri-apps/api/event"

const InstanceFolderButton = ({ onClick }: { onClick: () => void }) => {
	return (
		<Button size="icon" variant="instance-option" onClick={onClick}>
			<Folder className="w-6 h-6 text-white" />
		</Button>
	);
};

const InstancePlayButton = ({
	onClick,
	isRunning,
}: {
	onClick: () => void;
	isRunning: boolean;
}) => {
	return (
		<Button
			className={`bg-green-500 hover:bg-green-600 text-white 
                font-semibold rounded-md flex items-center gap-2 
                shadow transition disabled:opacity-50 disabled:cursor-not-allowed`}
			size="instance-play"
			onClick={onClick}
			disabled={isRunning}
		>
			<Play className="w-6 h-6" />
			<span>{isRunning ? 'Running...' : 'Play'}</span>
		</Button>
	);
};

const InstanceHeader = ({ instance }: { instance: Instance }) => {
  const [isRunning, setIsRunning] = useState(false);

  return (
    <div className="flex flex-col lg:flex-row items-start lg:items-center justify-between bg-neutral-800 rounded-2xl p-6 gap-6 shadow-lg">
      <div className="flex gap-6 items-center w-full lg:w-auto">
        <div className="relative group">
          <div className="w-20 h-20 sm:w-24 sm:h-24 bg-gradient-to-br from-[#424555] to-[#363a4a] rounded-xl flex justify-center items-center shrink-0 border-2 border-[#4a4f63]/50 group-hover:border-blue-400/30 transition-all duration-300">
            <Blocks className="w-10 h-10 sm:w-14 sm:h-14 text-blue-400 group-hover:text-blue-300 transition-colors duration-300" />
          </div>
          <div className="absolute inset-0 w-20 h-20 sm:w-24 sm:h-24 bg-blue-400/10 rounded-xl blur-md opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
        </div>
        <div className="flex flex-col gap-2">
          <h1 className="text-2xl sm:text-3xl font-bold break-all">{instance.name}</h1>
          <div className="flex gap-2">
            <div className="inline-block bg-blue-500/20 text-blue-400 text-sm font-semibold px-3 py-1 rounded-full w-fit">
              {instance.mod_loader}
            </div>
            <div className="inline-block bg-blue-500/20 text-blue-400 text-sm font-semibold px-3 py-1 rounded-full w-fit">
              {instance.mc_version}
            </div>
          </div>
        </div>
      </div>
      <div className="flex flex-wrap gap-2 w-full lg:w-auto justify-start lg:justify-end">
        <InstancePlayButton
          onClick={async () => {
            setIsRunning(true)
            try {
              ToastInfo("Instance has begun launching...")
              await launchInstance(instance.name)
              ToastSuccess("Instance has been closed successfully.")
            } finally {
              setIsRunning(false)
            }
          }}
          isRunning={isRunning}
        />
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button size="icon" variant="instance-option">
              <Ellipsis className="w-6 h-6 text-white" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent className="mt-2">
            <DropdownMenuItem onClick={async () => killInstance(instance.name)}>
              <FolderUp className="mr-1" /> Kill Instance
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
        <InstanceFolderButton onClick={async () => openInstanceFolder(instance.name)} />
      </div>
    </div>
  );
}

const TabNav = ({ tab, setTab }: {
  tab: "content" | "logs" | "saves" | "screenshots" | "console";
  setTab: (tab: "content" | "logs" | "saves" | "screenshots" | "console") => void;
}) => {
  const tabs = ["content", "logs", "saves", "screenshots", "console"] as const
  const { t: tr } = useTranslation("instance-page")

  return (
    <div className="mt-6 border-b border-neutral-700 flex gap-2 sm:gap-4 flex-wrap">
      {tabs.map((t) => (
        <button
          key={t}
          onClick={() => setTab(t)}
          className={`capitalize px-3 py-2 font-medium border-b-2 text-sm sm:text-base ${tab === t ? "text-blue-400 border-blue-400" : "text-neutral-400 border-transparent hover:text-white"
            } transition`}
        >
          {tr(`tabs.${t}`)}
        </button>
      ))}
    </div>
  );
}

const Tab = ({ tab, instance }: { tab: "content" | "logs" | "saves" | "screenshots" | "console", instance: Instance }) => {
  const [logs, setLogs] = useState<string[]>([]);
  const [gameInfo, setGameInfo] = useState<GameInfo>()
  const [isCopied, setIsCopied] = useState(false)

  useEffect(() => {
    const unlisten = listen<string>(`${instance.name}-console`, (event) => {
      setLogs((prev) => [...prev, event.payload]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  useEffect(() => {
    if (!instance) return
    const fetchAll = async () => {
      const gameInfo = await getGameInfo(instance.name)
      setGameInfo(gameInfo)
    }
    fetchAll()
  }, [instance])


    const copyLogsToClipboard = async () => {
    try {
      await navigator.clipboard.writeText(logs.join("\n"))
      setIsCopied(true)
      setTimeout(() => setIsCopied(false), 2000)
    } catch (err) {
      console.error("Failed to copy logs:", err)
    }
  }


  return (
    <div className="mt-6 p-4 sm:p-6 bg-neutral-800 rounded-xl text-neutral-300 text-base min-h-full">
      {tab === "content" && (
        <div className="flex flex-col items-center justify-center text-center py-8">
          <div className="w-16 h-16 bg-neutral-700 rounded-full flex items-center justify-center mb-6">
            <Download className="w-8 h-8 text-neutral-400" />
          </div>
          <h3 className="text-xl font-semibold text-white mb-3">No Content Installed</h3>
          <p className="text-neutral-400 mb-6 max-w-md">
            You haven't added any mods, resource packs, or other content to this instance yet. Get started by
            installing some content to enhance your gameplay experience.
          </p>
          <div className="flex flex-col sm:flex-row gap-3">
            <button className="bg-neutral-700 hover:bg-neutral-600 px-6 py-3 rounded-md text-white font-medium transition flex items-center gap-2">
              <Download className="w-4 h-4" />
              Browse Content
            </button>
          </div>
        </div>
      )}

      {tab === "logs" && (
        <div className="flex flex-col items-center justify-center text-center py-8">
          <div className="w-16 h-16 bg-neutral-700 rounded-full flex items-center justify-center mb-6">
            <FileText className="w-8 h-8 text-neutral-400" />
          </div>
          <h3 className="text-xl font-semibold text-white mb-3">Logs Coming Soon</h3>
          <p className="text-neutral-400 mb-6 max-w-md">
            Instance logs and crash reports will be displayed here to help you troubleshoot any issues.
          </p>
          <div className="bg-neutral-700/50 rounded-lg p-4 border border-neutral-600">
            <p className="text-sm text-neutral-400">
              💡 Tip: Logs will automatically appear here when you launch the instance
            </p>
          </div>
        </div>
      )}

      {tab === "saves" && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <Save className="w-5 h-5 text-blue-400" />
              <h2 className="text-lg font-semibold text-white">World Saves</h2>
            </div>
            {gameInfo?.worlds && gameInfo.worlds.length > 0 && (
              <span className="text-sm text-neutral-400">
                {gameInfo.worlds.length} world{gameInfo.worlds.length !== 1 ? "s" : ""}
              </span>
            )}
          </div>

          {gameInfo?.worlds && gameInfo.worlds.length > 0 ? (
            <div className="grid gap-3">
              {gameInfo.worlds.map((world, index) => (
                <div
                  key={index}
                  className="bg-neutral-700 rounded-xl shadow-lg hover:shadow-xl hover:bg-neutral-600/80 transition-all duration-200 cursor-pointer group"
                >
                  <div className="flex items-center p-4 gap-4">
                    <div className="relative">
                      <img
                        src={`data:image/png;base64,${world.icon}`}
                        alt={world.name}
                        className="w-16 h-16 object-cover rounded-lg border border-neutral-600 group-hover:border-neutral-500 transition-colors"
                      />
                      <div className="absolute -bottom-1 -right-1 w-6 h-6 bg-green-500 rounded-full flex items-center justify-center">
                        <Gamepad2 className="w-3 h-3 text-white" />
                      </div>
                    </div>
                    <div className="flex-1">
                      <h3 className="text-white font-semibold text-lg group-hover:text-blue-400 transition-colors">
                        {world.name}
                      </h3>
                      <div className="flex items-center gap-4 mt-1 text-sm text-neutral-400">
                        <span className="flex items-center gap-1">
                          <Calendar className="w-3 h-3" />
                          Last played today
                        </span>
                        <span className="flex items-center gap-1">
                          <Clock className="w-3 h-3" />
                          2h 34m played
                        </span>
                      </div>
                      <div className="mt-2">
                        <span className="inline-block bg-green-500/20 text-green-400 text-xs font-medium px-2 py-1 rounded-full">
                          Survival
                        </span>
                      </div>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center text-center py-12">
              <div className="w-16 h-16 bg-neutral-700 rounded-full flex items-center justify-center mb-6">
                <Save className="w-8 h-8 text-neutral-400" />
              </div>
              <h3 className="text-xl font-semibold text-white mb-3">No Worlds Found</h3>
              <p className="text-neutral-400 max-w-md">
                Your world saves will appear here once you start playing. Create a new world or import existing saves
                to get started.
              </p>
            </div>
          )}
        </div>
      )}

      {tab === "screenshots" && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center gap-2">
              <ImageIcon className="w-5 h-5 text-blue-400" />
              <h2 className="text-lg font-semibold text-white">Screenshots</h2>
            </div>
            {gameInfo?.screenshots && gameInfo.screenshots.length > 0 && (
              <span className="text-sm text-neutral-400">
                {gameInfo.screenshots.length} screenshot{gameInfo.screenshots.length !== 1 ? "s" : ""}
              </span>
            )}
          </div>

          {gameInfo?.screenshots && gameInfo.screenshots.length > 0 ? (
            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-4">
              {gameInfo.screenshots.map((screenshot, index) => (
                <div
                  key={index}
                  className="bg-neutral-700 rounded-lg overflow-hidden shadow-lg hover:shadow-xl transition-all duration-200 group cursor-pointer"
                >
                  <div className="relative overflow-hidden">
                    <img
                      src={`data:image/png;base64,${screenshot.screenshot}`}
                      alt={screenshot.name}
                      className="w-full h-48 sm:h-52 object-cover group-hover:scale-105 transition-transform duration-300"
                    />
                    <div className="absolute inset-0 bg-black/0 group-hover:bg-black/20 transition-colors duration-200" />
                    <div className="absolute top-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
                      <div className="bg-black/50 rounded-full p-1">
                        <ImageIcon className="w-4 h-4 text-white" />
                      </div>
                    </div>
                  </div>
                  <div className="p-3">
                    <p className="text-neutral-300 text-sm truncate group-hover:text-white transition-colors">
                      {screenshot.name}
                    </p>
                    {/* <p className="text-xs text-neutral-500 mt-1">Taken recently</p> */}
                  </div>
                </div>
              ))}
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center text-center py-12">
              <div className="w-16 h-16 bg-neutral-700 rounded-full flex items-center justify-center mb-6">
                <ImageIcon className="w-8 h-8 text-neutral-400" />
              </div>
              <h3 className="text-xl font-semibold text-white mb-3">No Screenshots</h3>
              <p className="text-neutral-400 max-w-md mb-4">
                Your screenshots will appear here. Press F2 in-game to capture those epic moments and beautiful
                builds.
              </p>
              <div className="bg-neutral-700/50 rounded-lg p-4 border border-neutral-600">
                <p className="text-sm text-neutral-400">
                  💡 Tip: Screenshots are automatically saved to your instance folder
                </p>
              </div>
            </div>
          )}
        </div>
      )}

      {tab === "console" && (
          <div className="space-y-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Terminal className="w-5 h-5 text-blue-400" />
                <h2 className="text-lg font-semibold text-white">Console</h2>
              </div>
              <Button
                onClick={copyLogsToClipboard}
                variant="outline"
                size="sm"
                className="border-neutral-600 bg-neutral-700/50 hover:bg-neutral-700 text-neutral-300 hover:text-white transition-all duration-200"
              >
                {isCopied ? (
                  <>
                    <Check className="w-4 h-4 mr-2 text-green-400" />
                    Copied!
                  </>
                ) : (
                  <>
                    <Copy className="w-4 h-4 mr-2" />
                    Copy Logs
                  </>
                )}
              </Button>
            </div>

            <div className="bg-black rounded-lg border border-neutral-800 min-h-[500px] max-h-[500px] overflow-hidden shadow-2xl">
              <div className="bg-neutral-900 border-b border-neutral-800 px-4 py-2 flex items-center gap-3">
                <div className="flex items-center gap-2 text-neutral-400 text-sm">
                  <Terminal className="w-4 h-4" />
                  <span>Minecraft Console</span>
                </div>
              </div>

              <div className="p-4 font-mono text-sm overflow-y-auto max-h-[440px] bg-black">
                <div className="space-y-1">
                  {logs.length > 0 ? (
                    logs.map((line, idx) => (
                      <div
                        key={idx}
                        className="flex items-start gap-2 group hover:bg-neutral-900/30 px-2 py-0.5 rounded transition-colors"
                      >
                        <span className="text-neutral-600 text-xs mt-0.5 font-mono tabular-nums min-w-[60px]">
                          {String(idx + 1).padStart(3, "0")}
                        </span>
                        <span className="text-green-400 break-all leading-relaxed">{line}</span>
                      </div>
                    ))
                  ) : (
                    <div className="space-y-2 text-neutral-500">
                      <div className="flex items-start gap-2">
                        <span className="text-neutral-600 text-xs mt-0.5 font-mono tabular-nums min-w-[60px]">001</span>
                        <span className="text-blue-400">[INFO] Console will output here...</span>
                      </div>
                    </div>
                  )}
                </div>

              </div>
            </div>
          </div>
        )}
    </div>
  );
}

export const InstancePage = () => {
  const { name } = useParams<{ name: string }>()
  const [instance, setInstance] = useState<Instance>()
  const [tab, setTab] = useState<"content" | "logs" | "saves" | "screenshots" | "console">("content")
  const [isLoading, setIsLoading] = useState(false)

  useEffect(() => {
    const fetchAll = async () => {
      setIsLoading(true)
      const instances = await getInstances()
      const match = instances?.find((inst) => inst.name === name)
      setInstance(match)
      setIsLoading(false)
    }
    fetchAll()
  }, [])

  if (isLoading) {
    return (
        <div className="flex flex-col h-full items-center justify-center py-16 px-6">
          <div className="relative mb-4">
            <Loader2 className="animate-spin w-8 h-8 text-blue-400" />
            <div className="absolute inset-0 w-8 h-8 border-2 border-blue-400/20 rounded-full animate-pulse"></div>
          </div>
          <h3 className="text-lg font-semibold text-white mb-2">Loading the instance</h3>
          <p className="text-neutral-400 text-center">
            Getting the instance data!
          </p>
        </div>
    )
  }

  if (!instance) {
    return (
      <div className="flex flex-col h-full items-center justify-center py-16 px-6">
        <div className="w-20 h-20 bg-red-500/10 rounded-full flex items-center justify-center mb-6">
          <AlertCircle className="w-10 h-10 text-red-400" />
        </div>


        <h3 className="text-xl font-semibold text-red-400 mb-3">Failed to find instance</h3>
        <p className="text-neutral-400 text-center max-w-md mb-4">
          Make sure the instance exists. Please try again later.
        </p>
      </div>
    )
  }

  return (
    <div className="w-full px-4 md:px-8 py-6 min-h-screen text-white">
      <InstanceHeader instance={instance} />

      <TabNav tab={tab} setTab={setTab} />

      <Tab tab={tab} instance={instance} />
    </div>
  )
}
