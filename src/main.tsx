import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './app.tsx'

import WindowBar from './components/window/bar.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('app') as HTMLElement).render(
    <React.StrictMode>
        <WindowBar />
        <App />
    </React.StrictMode>,
)
