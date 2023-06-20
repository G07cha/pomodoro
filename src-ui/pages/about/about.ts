import { appWindow } from '@tauri-apps/api/window';

import * as theme from '../../utils/theme';
import { disableContextMenu } from '../../utils/dom';

appWindow.emit('window_loaded');

theme.followSystemTheme();
disableContextMenu();
