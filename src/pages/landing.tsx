// export default function LandingPage(){
//     return (
//         <div>
//            <svg width="128" height="128" viewBox="0 0 128 128" xmlns="http://www.w3.org/2000/svg">
//   <g stroke="#0F172A" stroke-width="6" stroke-linejoin="round" fill="none">
//     {/*  主星形  */}
//     <polygon points="64,10 78,50 118,64 78,78 64,118 50,78 10,64 50,50"/>
    
//     {/*  中心圆  */}
//     <circle cx="64" cy="64" r="14"/>
//   </g>
//       </svg>
//         </div>
//     )
// }

import { useNavigate } from 'react-router-dom';

export default function LandingPage() {
  const navigate = useNavigate();

  return (
    <div className="flex flex-col items-center justify-center min-h-screen bg-gray-50 p-6 text-center">
      {/* Logo */}
      <svg
        width="128"
        height="128"
        viewBox="0 0 128 128"
        xmlns="http://www.w3.org/2000/svg"
        className="mb-6"
      >
        <g stroke="#0F172A" strokeWidth="6" strokeLinejoin="round" fill="none">
          {/* 主星形 */}
          <polygon points="64,10 78,50 118,64 78,78 64,118 50,78 10,64 50,50" />
          {/* 中心圆 */}
          <circle cx="64" cy="64" r="14" />
        </g>
      </svg>

      {/* 标题 */}
      <h1 className="text-4xl font-bold text-gray-900 mb-4">
        Welcome to Merak
      </h1>

      {/* 描述 */}
      <p className="text-gray-700 mb-8 max-w-md">
        Merak is an open-source community version of Feishu Project. Explore, collaborate, and get started quickly with our intuitive interface.
      </p>

      {/* 按钮 */}
      <div className="flex gap-4">
        <button
          onClick={() => navigate('/app')}
          className="px-6 py-3 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition"
        >
          Enter App
        </button>
        <a
          href="https://www.fumadocs.dev/"
          target="_blank"
          className="px-6 py-3 border border-gray-300 rounded-md hover:bg-gray-100 transition"
        >
          Docs
        </a>
      </div>
    </div>
  );
}
