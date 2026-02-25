import { prisma } from '../lib/prisma';

export class RoundService {
  async createRound(startPrice: number, mode: number) {
    return prisma.round.create({
      data: { startPrice, mode, status: 'ACTIVE' }
    });
  }

  async getRound(id: string) {
    return prisma.round.findUnique({ where: { id } });
  }

  async resolveRound(id: string, finalPrice: number) {
    return prisma.round.update({
      where: { id },
      data: { finalPrice, status: 'RESOLVED' }
    });
  }

  async getActiveRound() {
    return prisma.round.findFirst({ where: { status: 'ACTIVE' } });
  }
}
