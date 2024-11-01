namespace App01
{
    public class FyroxImpls
    {
        internal static readonly FyroxManualBindings.CreateScriptInstance CreateScriptInstance = (it) =>
        {
            throw new Exception("hello 123");
        };
    }
}