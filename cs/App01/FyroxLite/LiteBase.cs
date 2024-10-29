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
    internal float Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static float? ToFacade(in float_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
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
            return new float_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = __item;
        return new float_optional { Value = __item_from_facade.Value, HasValue = true };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct string_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal string Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe string ToFacade(in string_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static string_result FromFacade(in string self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new string_result {Ok = 1, Value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Sequential)]
internal struct UserScript_optional
{
    internal UserScript Value;
    internal bool HasValue;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static object? ToFacade(in UserScript_optional value)
    {
        if (value.HasValue)
        {
            var __item = value.Value;
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
            return new UserScript_optional { Value = default, HasValue = false };
        }
        var __item = value;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_optional { Value = __item_from_facade, HasValue = true };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal UserScript Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe object ToFacade(in UserScript_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = UserScript.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static UserScript_result FromFacade(in object self)
    {
        var __item = self;
        var __item_from_facade = UserScript.FromFacade(__item);
        return new UserScript_result {Ok = 1, Value = __item_from_facade};
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct UserScript_optional_result
{
    [FieldOffset(0)]
    internal int Ok;

    [FieldOffset(sizeof(int))]
    internal UserScript_optional Value;

    [FieldOffset(sizeof(int))]
    internal string Err;

    internal static unsafe object? ToFacade(in UserScript_optional_result self)
    {
        if (self.Ok != 0)
        {
            var __item = self.Value;
            var __item_to_facade = UserScript_optional.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(self.Err);
    }

    internal static UserScript_optional_result FromFacade(in object? self)
    {
        var __item = self;
        var __item_from_facade = UserScript_optional.FromFacade(__item);
        return new UserScript_optional_result {Ok = 1, Value = __item_from_facade};
    }
}