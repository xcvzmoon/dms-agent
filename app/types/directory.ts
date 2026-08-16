import * as v from 'valibot';

export const directorySchema = v.object({
  id: v.optional(v.string()),
  path: v.pipe(v.string(), v.minLength(1, 'Select a directory')),
  dmsEndpoint: v.pipe(v.string(), v.trim()),
  includePatterns: v.pipe(v.string(), v.trim()),
  isActive: v.boolean(),
  lastSync: v.nullable(v.string()),
});

export type Directory = v.InferOutput<typeof directorySchema>;
