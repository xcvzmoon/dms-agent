import * as v from 'valibot';

export const syncLogEntrySchema = v.object({
  id: v.string(),
  directoryPath: v.string(),
  file: v.string(),
  fileName: v.string(),
  dmsEndpoint: v.string(),
  documentIds: v.array(v.number()),
  createdAt: v.number(),
});

export type SyncLogEntry = v.InferOutput<typeof syncLogEntrySchema>;
