import { type Component, createEffect, createSignal, Show } from 'solid-js';
import { Route, Routes } from '@solidjs/router';
import { platform } from '@tauri-apps/plugin-os';

import WindowBar from './components/window/bar';
import IndexPage from './pages/index';

import ThemeContextProvider from './context/theme';
import SettingsContextProvider from './context/settings';
import AudioContextProvider from './context/audio';

const App: Component = () => {
  const [mobile, setMobile] = createSignal(false);

  createEffect(async () => {
    await platform().then((platform) => {
      if (platform === "android" || platform === "ios") {
        setMobile(true);
      }
    });

    const handleContextMenu = (e: any) => { e.preventDefault() };

    document.addEventListener("contextmenu", handleContextMenu)

    return () => {
      document.removeEventListener("contextmenu", handleContextMenu)
    }
  });

  return (
    <ThemeContextProvider>
      <main class='w-full h-full bg-white dark:bg-black text-black dark:text-white'>
        <nav id='nav' class='fixed z-10 top-0 w-full h-[36px] data-[mobile=true]:hidden' data-mobile={mobile()}>
          <WindowBar />
        </nav>
        <SettingsContextProvider>
          <AudioContextProvider>
            <Routes>
              <Route path='/' element={<IndexPage />} />
            </Routes>
          </AudioContextProvider>
        </SettingsContextProvider >
      </main>
    </ThemeContextProvider>
  );
};

export default App;
