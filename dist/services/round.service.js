"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.RoundService = void 0;
const prisma_1 = require("../lib/prisma");
class RoundService {
    async createRound(startPrice, mode) {
        return prisma_1.prisma.round.create({
            data: { startPrice, mode, status: 'ACTIVE' }
        });
    }
    async getRound(id) {
        return prisma_1.prisma.round.findUnique({ where: { id } });
    }
    async resolveRound(id, finalPrice) {
        return prisma_1.prisma.round.update({
            where: { id },
            data: { finalPrice, status: 'RESOLVED' }
        });
    }
    async getActiveRound() {
        return prisma_1.prisma.round.findFirst({ where: { status: 'ACTIVE' } });
    }
}
exports.RoundService = RoundService;
