import * as fs from 'fs';
import { compileFromFile } from 'json-schema-to-typescript';

(async () => {
  const schemaPath = 'src/schemas/curated-listing.json';
  const modelsPath = 'src/models/models.d.ts';
  const ts = await compileFromFile(schemaPath);
  fs.writeFileSync(modelsPath, ts);
})();
