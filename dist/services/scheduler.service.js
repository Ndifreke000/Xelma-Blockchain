"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SchedulerService = void 0;
const prisma_1 = require("../lib/prisma");
class SchedulerService {
    async scheduleRoundResolution(roundId, resolveAt) {
        return prisma_1.prisma.scheduledTask.create({
            data: { roundId, type: 'RESOLVE_ROUND', executeAt: resolveAt }
        });
    }
    async getPendingTasks() {
        return prisma_1.prisma.scheduledTask.findMany({
            where: {
                status: 'PENDING',
                executeAt: { lte: new Date() }
            }
        });
    }
    async completeTask(id) {
        return prisma_1.prisma.scheduledTask.update({
            where: { id },
            data: { status: 'COMPLETED' }
        });
    }
}
exports.SchedulerService = SchedulerService;
