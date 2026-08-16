import * as v from 'valibot';

export const watchActionSchema = v.picklist(['ADD', 'CHANGE', 'UNLINK']);

export const watchLogEntrySchema = v.object({
  id: v.string(),
  directoryPath: v.string(),
  directoryId: v.nullable(v.string()),
  file: v.string(),
  dmsEndpoint: v.string(),
  isSync: v.boolean(),
  createdAt: v.number(),
  action: watchActionSchema,
});

export type WatchAction = v.InferOutput<typeof watchActionSchema>;
export type WatchLogEntry = v.InferOutput<typeof watchLogEntrySchema>;
