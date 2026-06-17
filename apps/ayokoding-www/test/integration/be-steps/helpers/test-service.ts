import { ContentService } from "@/features/content/shell/service";
import { FileSystemContentRepository } from "@/features/content/shell/repository-fs";
import path from "node:path";

const contentDir = path.resolve(process.cwd(), "content");
const repository = new FileSystemContentRepository(contentDir);

export const testContentService = new ContentService(repository);
