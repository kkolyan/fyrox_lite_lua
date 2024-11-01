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
using FyroxLite.LiteBase;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;


[StructLayout(LayoutKind.Sequential)]
internal struct float_optional
{
    private float value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static float? ToFacade(in float_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static float_optional FromFacade(in float? value)
    {
        if (value == null)
        {
            return new float_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new float_optional { value = __item_from_facade.Value, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeString_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private NativeString value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe string ToFacade(in NativeString_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = NativeString.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static NativeString_result FromFacade(in string self)
    {
        var __item = self;
        var __item_from_facade = NativeString.FromFacade(__item);
        return new NativeString_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct UserScript_optional
{
    private UserScript value;
    private int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static object? ToFacade(in UserScript_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = UserScript.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static UserScript_optional FromFacade(in object? value)
    {
        if (value == null)
        {
            return new UserScript_optional { value = default, has_value = 0 };
        }
        var __item = value;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private UserScript value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe object ToFacade(in UserScript_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = UserScript.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static UserScript_result FromFacade(in object self)
    {
        var __item = self;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_result {ok = 1, value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_optional_result
{
    [FieldOffset(0)]
    private int ok;

    [FieldOffset(sizeof(int))]
    private UserScript_optional value;

    [FieldOffset(sizeof(int))]
    private string err;

    internal static unsafe object? ToFacade(in UserScript_optional_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value;
            var __item_to_facade = UserScript_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.err);
    }

    internal static UserScript_optional_result FromFacade(in object? self)
    {
        var __item = self;
        var __item_from_facade = UserScript_optional.FromFacade(__item);
        return new UserScript_optional_result {ok = 1, value = __item_from_facade};
    }
}