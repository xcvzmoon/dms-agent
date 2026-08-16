export type UploadItem = {
  id: string;
  file: string;
  directoryPath: string;
  dmsEndpoint?: string;
  fileType?: string;
  fileName?: string;
};

export type UploadOutcome = {
  dmsEndpoint: string;
  documentIds: number[];
};
