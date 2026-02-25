import { prisma } from '../lib/prisma';

export class SchedulerService {
  async scheduleRoundResolution(roundId: string, resolveAt: Date) {
    return prisma.scheduledTask.create({
      data: { roundId, type: 'RESOLVE_ROUND', executeAt: resolveAt }
    });
  }

  async getPendingTasks() {
    return prisma.scheduledTask.findMany({
      where: {
        status: 'PENDING',
        executeAt: { lte: new Date() }
      }
    });
  }

  async completeTask(id: string) {
    return prisma.scheduledTask.update({
      where: { id },
      data: { status: 'COMPLETED' }
    });
  }
}
