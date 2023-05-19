// (c) 2023 - dromzeh (Marcel) <marcel@dromzeh.dev>
// Licensed under the MIT license - https://mit.dromzeh.dev/

// importing the required modules
const Registry = require('winreg');
const readline = require('readline');
const { Buffer } = require('buffer');

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
    console.log('Hi3 regkey does not exist!');
    return;
  }

  console.log('Found Hi3 regkey, checking for PersonalGraphicsSetting subkey...');

  regKey.values((err, items) => {
    if (err) {
      console.log(err);
      return;
    }

    // find the value we want to modify
    const value = items.find(item => item.type === 'REG_BINARY' && item.name.includes('GENERAL_DATA_V2_PersonalGraphicsSettingV2'));

    if (!value) {
      console.log('Did not find value');
      return;
    }

    console.log(`Found Graphics Settings Binary value with name '${value.name}'`);

    //  data parsing & conversion logic
    const hexData = value.value.toString('hex');
    const buffer = Buffer.from(hexData, 'hex');
    const dataString = buffer.toString();

    const printableAscii = /[\x20-\x7E]+/g;
    const cleanDataString = dataString.match(printableAscii).join('');
    const graphicsSettings = JSON.parse(cleanDataString);

    console.log(`Current FPS out of level: ${graphicsSettings.TargetFrameRateForOthers}`);
    console.log(`Current FPS in level: ${graphicsSettings.TargetFrameRateForInLevel}`);

    const rl = readline.createInterface({
      input: process.stdin,
      output: process.stdout
    });

    // prompt user for new FPS values
    rl.question('Enter the desired FPS out of level: ', (combatFPS) => {
      rl.question('Enter the desired FPS in level: ', (outOfCombatFPS) => {
        graphicsSettings.TargetFrameRateForInLevel = parseInt(combatFPS);
        graphicsSettings.TargetFrameRateForOthers = parseInt(outOfCombatFPS);

        console.log(`The modified graphics settings are: ${JSON.stringify(graphicsSettings)}`);

        const modifiedDataString = JSON.stringify(graphicsSettings);
        const modifiedBuffer = Buffer.from(modifiedDataString);
        const modifiedHexData = modifiedBuffer.toString('hex');

        regKey.set(value.name, Registry.REG_BINARY, modifiedHexData, (err) => {
          if (err) {
            console.log(err);
            return;
          }

          console.log(`Updated value ${value.name} to ${modifiedHexData}`);
          rl.close();
        });
      });
    });
  });
});