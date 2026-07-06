-- 启动器可见性（替代 hide_on_process_start 的布尔值，提供更细粒度控制）
-- 0=ExitImmediately, 2=HideAndExit, 3=HideAndReopen, 4=MinimizeAndReopen, 5=DoNothing
ALTER TABLE settings ADD COLUMN launcher_visibility INTEGER NOT NULL DEFAULT 5;
-- 迁移旧数据：hide_on_process_start=true → launcher_visibility=4 (MinimizeAndReopen)
UPDATE settings SET launcher_visibility = CASE WHEN hide_on_process_start = 1 THEN 4 ELSE 5 END;

-- 进程优先级 (0=AboveNormal, 1=Normal, 2=BelowNormal, 3=High, 4=RealTime)
ALTER TABLE settings ADD COLUMN process_priority INTEGER NOT NULL DEFAULT 1;

-- 渲染器 (0=Default, 1=LLVMPipe软渲染, 2=DirectX12, 3=Vulkan/Zink)
ALTER TABLE settings ADD COLUMN renderer INTEGER NOT NULL DEFAULT 0;

-- 游戏参数尾部（拼合在启动参数末尾）
ALTER TABLE settings ADD COLUMN extra_game_args JSONB NOT NULL DEFAULT '[]';

-- IP协议偏好 (0=PreferV4, 1=Default, 2=PreferV6)
ALTER TABLE settings ADD COLUMN preferred_ip_stack INTEGER NOT NULL DEFAULT 1;

-- 自定义信息（显示在游戏左下角/F3调试页）
ALTER TABLE settings ADD COLUMN custom_info TEXT NOT NULL DEFAULT '';

-- 游戏窗口标题（支持替换标记）
ALTER TABLE settings ADD COLUMN window_title TEXT NOT NULL DEFAULT '';

-- 内存分配模式 (0=Auto自动, 1=Custom自定义)
ALTER TABLE settings ADD COLUMN memory_allocation_mode INTEGER NOT NULL DEFAULT 1;

-- 高性能显卡优先
ALTER TABLE settings ADD COLUMN set_gpu_preference INTEGER NOT NULL DEFAULT 0;

-- 使用 java.exe 而不是 javaw.exe
ALTER TABLE settings ADD COLUMN use_java_exe INTEGER NOT NULL DEFAULT 0;

-- 启动前命令等待完成
ALTER TABLE settings ADD COLUMN pre_launch_wait INTEGER NOT NULL DEFAULT 1;

-- 禁用 Java Launch Wrapper (theseus.jar 的 javaagent 注入)
ALTER TABLE settings ADD COLUMN disable_java_launch_wrapper INTEGER NOT NULL DEFAULT 0;

-- 禁用 LegacyFix（老版本兼容性修复）
ALTER TABLE settings ADD COLUMN disable_legacy_fix INTEGER NOT NULL DEFAULT 0;

-- 禁用 LWJGL Unsafe Agent
ALTER TABLE settings ADD COLUMN disable_lwjgl_unsafe_agent INTEGER NOT NULL DEFAULT 0;
