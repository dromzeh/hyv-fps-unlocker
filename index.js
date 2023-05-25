// (c) 2023 - dromzeh (Marcel) <marcel@dromzeh.dev>
// Licensed under the MIT license - https://mit.dromzeh.dev/

// importing the required modules
import Registry from 'winreg';
import readline from 'readline';
import { Buffer } from 'buffer';
import chalk from 'chalk';

const regKey = new Registry({
  hive: Registry.HKCU,
  key: '\\Software\\miHoYo\\Honkai Impact 3rd' // hi3 regkey
});

regKey.keyExists((err, exists) => {
  if (err) {
    console.log(err);
    return;
  }

  if (!exists) {
    console.log(`${chalk.gray(`[${chalk.red('-')}]`)} Could not find Hi3 regkey!`);
    return;
  }

  console.log(`${chalk.gray(`[${chalk.green('+')}]`)} Found Hi3 regkey, checking for PersonalGraphicsSetting subkey..`);

  regKey.values((err, items) => {
    if (err) {
      console.log(err);
      return;
    }

    // find the value we want to modify
    const value = items.find(item => item.type === 'REG_BINARY' && item.name.includes('GENERAL_DATA_V2_PersonalGraphicsSettingV2'));

    if (!value) {
      console.log(`${chalk.gray(`[${chalk.red('-')}]`)} Could not find Graphics Settings Binary value!`);
      return;
    }

    console.log(`${chalk.gray(`[${chalk.green('+')}]`)} Found Graphics Settings Binary value ${chalk.cyan(value.name)}!\n`);

    //  data parsing & conversion logics
    const hexData = value.value.toString('hex');
    const buffer = Buffer.from(hexData, 'hex');
    const dataString = buffer.toString();

    const printableAscii = /[\x20-\x7E]+/g;
    const cleanDataString = dataString.match(printableAscii).join('');
    const graphicsSettings = JSON.parse(cleanDataString);


    // print current FPS values
    console.log(`${chalk.gray(`[${chalk.green('+')}]`)} Current FPS in level: ${chalk.cyan(graphicsSettings.TargetFrameRateForInLevel)}`);
    console.log(`${chalk.gray(`[${chalk.green('+')}]`)} Current FPS out of level: ${chalk.cyan(graphicsSettings.TargetFrameRateForOthers)} \n`);

    // save old FPS values
    const oldFpsValues = {
      inLevel: graphicsSettings.TargetFrameRateForInLevel,
      outOfLevel: graphicsSettings.TargetFrameRateForOthers
    }

    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    // prompt user for new FPS values
    rl.question(`${chalk.gray(`[${chalk.cyan('!')}]`)} Enter new FPS in level: `, (combatFPS) => {
      rl.question(`${chalk.gray(`[${chalk.cyan('!')}]`)} Enter new FPS out of level: `, (outOfCombatFPS) => {
        graphicsSettings.TargetFrameRateForInLevel = parseInt(combatFPS);
        graphicsSettings.TargetFrameRateForOthers = parseInt(outOfCombatFPS);

        const modifiedDataString = JSON.stringify(graphicsSettings);
        const modifiedBuffer = Buffer.from(modifiedDataString);
        const modifiedHexData = modifiedBuffer.toString('hex');

        regKey.set(value.name, Registry.REG_BINARY, modifiedHexData, (err) => {
          if (err) {
            console.log(`${chalk.gray(`[${chalk.red('-')}]`)} Could not update value ${value.name}!`);
            console.log(err);
            return;
          }

          console.log(`\n${chalk.gray(`[${chalk.green('+')}]`)} Updated FPS from ${chalk.cyan(oldFpsValues.inLevel)} to ${chalk.cyan(combatFPS)} in level and from ${chalk.cyan(oldFpsValues.outOfLevel)} to ${chalk.cyan(outOfCombatFPS)} out of level!`);
          console.log(`\n${chalk.gray(`[${chalk.cyan('!')}]`)} New configuration: ${chalk.cyan(JSON.stringify(graphicsSettings))}`);

          rl.close();
        });
      });
    });
  });
});