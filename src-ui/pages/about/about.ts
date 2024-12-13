import { getVersion } from '@tauri-apps/api/app';

import { disableContextMenu, getElementByIdOrThrow } from '../../utils/dom';
import * as theme from '../../utils/theme';

theme.followSystemTheme();
disableContextMenu();

const versionElement = getElementByIdOrThrow('version');
versionElement.innerText = await getVersion();
