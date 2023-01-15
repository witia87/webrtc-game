import { Command, CommandType } from './messages/commands';

export class CommandsEncoder {

  public static encode(commandType: CommandType,
                       commandPayloadBytes: Uint8Array): Uint8Array {

    return Command.encode({
      commandType: commandType,
      commandPayload: commandPayloadBytes,
    }).finish();
  }
}
