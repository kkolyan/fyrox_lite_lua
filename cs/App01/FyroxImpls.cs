namespace App01
{
    public class FyroxImpls
    {
        public static readonly FyroxManualBindings.CreateScriptInstance CreateScriptInstance = (it) =>
        {
            throw new Exception("hello 123");
        };
    }
}