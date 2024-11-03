// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite.Internal;
using FyroxLite.LiteInput;
using FyroxLite.LiteLog;
using FyroxLite.LiteMath;
using FyroxLite.LiteNode;
using FyroxLite.LitePhysics;
using FyroxLite.LitePlugin;
using FyroxLite.LitePrefab;
using FyroxLite.LiteScene;
using FyroxLite.LiteUi;
using FyroxLite.LiteWindow;
using System.Numerics;
using System.Drawing;
using FyroxLite.LiteBase;
using FyroxLite.Internal;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite.Internal;

using System.Runtime.InteropServices;
using FyroxLite;
using FyroxLite.LiteMath;

internal partial class FyroxNativeGlobal {
    

    [LibraryImport("../../../../../target/debug/libfyrox_c.dylib", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static partial void init_fyrox(NativeScriptedApp app);

    internal delegate void LoadScripts();

    internal delegate void NodeOnUpdate(NativeInstanceId thiz, float dt);

    internal delegate void NodeOnInit(NativeInstanceId thiz);

    internal delegate void NodeOnDeinit(NativeInstanceId thiz);

    internal delegate void NodeOnStart(NativeInstanceId thiz);

    internal delegate void NodeOnMessage(NativeInstanceId thiz, UserScriptMessage message);

    internal delegate void GameOnInit(NativeInstanceId thiz);

    internal delegate void GameOnUpdate(NativeInstanceId thiz);

    internal delegate NativeInstanceId CreateScriptInstance(NativeClassId thiz);

    internal delegate void SetProperty(NativeInstanceId thiz, int property, NativeValue value);
}