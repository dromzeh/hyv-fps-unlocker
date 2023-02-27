// (c) 2023 - dromzeh (Marcel) <marcel@dromzeh.dev>
// Licensed under the MIT license - https://mit.dromzeh.dev/

// importing the required modules
const Registry = require('winreg');
const readline = require('readline');
const { Buffer } = require('buffer');

// creating a new registry key object for the Hi3 regkey
const regKey = new Registry({
  hive: Registry.HKCU,
  key: '\\Software\\miHoYo\\Honkai Impact 3rd' // The path to the Hi3 regkey.
});

// checking if the Hi3 regkey exists
regKey.keyExists(function(err, exists) {
  if (err) {
    console.log(err);
  } else {
    if (exists) {
      console.log('Found Hi3 regkey, checking for PersonalGraphicsSetting subkey...');

      // checking if the PersonalGraphicsSetting subkey exists
      regKey.values(function(err, items) {
        if (err) {
          console.log(err);
        } else {
          const value = items.find(item => item.type === 'REG_BINARY' && item.name.includes('GENERAL_DATA_V2_PersonalGraphicsSettingV2'));
          if (value) {
            // Found the binary value, now we can start parsing the data
            console.log(`Found Graphics Settings Binary value with name '${value.name}'\n`);

            // Convert the hex data to a string
            const hexData = value.value.toString('hex');
            const buffer = Buffer.from(hexData, 'hex');
            const dataString = buffer.toString();

            //console.log(`The data in the value is: ${dataString}`);

            // Clean the data string to remove all non-printable characters, then parse it to JSON
            const printableAscii = /[\x20-\x7E]+/g;
            const cleanDataString = dataString.match(printableAscii).join('');
            const graphicsSettings = JSON.parse(cleanDataString);

            // Print the current FPS settings
            console.log(`Current FPS out of combat: ${graphicsSettings.TargetFrameRateForOthers}`);
            console.log(`Current FPS in combat: ${graphicsSettings.TargetFrameRateForInLevel}\n`);

            // Ask the user for the desired FPS settings
            const rl = readline.createInterface({
              input: process.stdin,
              output: process.stdout
            });

            rl.question('Enter the desired FPS in combat: ', (combatFPS) => {
              rl.question('Enter the desired FPS outside of combat: ', (outOfCombatFPS) => {
                graphicsSettings.TargetFrameRateForInLevel = parseInt(combatFPS);
                graphicsSettings.TargetFrameRateForOthers = parseInt(outOfCombatFPS);

                console.log(`\nThe modified graphics settings are: ${JSON.stringify(graphicsSettings)}\n`);

                // Converts the modified object back to hex
                const modifiedDataString = JSON.stringify(graphicsSettings);
                const modifiedBuffer = Buffer.from(modifiedDataString);
                const modifiedHexData = modifiedBuffer.toString('hex');

                // Updating the binary value with the new hex data
                regKey.set(value.name, Registry.REG_BINARY, modifiedHexData, function(err) {
                  if (err) {
                    console.log(err);
                  } else {
                    console.log(`Updated value ${value.name} to ${modifiedHexData}\n`);
                  }
                });

                rl.close();
              });
            });
          } else {
            console.log('Did not find value\n');
          }
        }
      });
    } else {
      console.log('Hi3 regkey does not exist!\n');
    }
  }
});