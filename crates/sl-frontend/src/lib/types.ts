enum VersionType {
    OldBeta,
    OldAlpha,
    Release,
    Snapshot
};

interface InstallationInfo {
    type: VersionType,
    version: string
}

export interface Installation {
    name: string,
    info: InstallationInfo
}

export type InstanceCardProps = {
  title: string;
  version: string;
  modLoader?: string;
  modCount?: number;
  lastPlayed: string;
  image: string;
  favorite?: boolean;
};