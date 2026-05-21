export type CleanupDivergenceRow = {
  pid: number;
  executableBasename: string;
  cmdPreview: string;
};

export type CleanupTerminateResult = {
  pid: number;
  ok: boolean;
  message: string;
};
