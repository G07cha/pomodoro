import { getVersion } from '@tauri-apps/api/app';

import * as theme from '../../utils/theme';
import { disableContextMenu, getElementByIdOrThrow } from '../../utils/dom';

theme.followSystemTheme();
disableContextMenu();

const versionElement = getElementByIdOrThrow('version');
versionElement.innerText = await getVersion();
