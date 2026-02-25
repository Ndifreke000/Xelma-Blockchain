import { test, expect } from '@jest/globals';
import * as fs from 'fs';
import * as path from 'path';

const findPrismaInstances = (dir: string): string[] => {
  const files: string[] = [];
  fs.readdirSync(dir, { withFileTypes: true }).forEach(entry => {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory() && !['node_modules', 'dist', '__tests__'].includes(entry.name)) {
      files.push(...findPrismaInstances(fullPath));
    } else if (entry.isFile() && /\.ts$/.test(entry.name) && !entry.name.includes('.test.')) {
      const content = fs.readFileSync(fullPath, 'utf-8');
      if (content.includes('new PrismaClient()') && !fullPath.includes('lib/prisma')) {
        files.push(fullPath);
      }
    }
  });
  return files;
};

test('no direct PrismaClient instantiation', () => {
  const violations = findPrismaInstances(path.join(__dirname, '..'));
  expect(violations).toEqual([]);
});
