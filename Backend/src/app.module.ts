import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { AuthModule } from './auth/auth.module';
import { JewelryModule } from './jewelry/jewelry.module';
import { ContractsModule } from './contracts/contracts.module';

@Module({
  imports: [AuthModule, JewelryModule, ContractsModule],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
