using System.Text.Json;
using System.Text.Json.Serialization;
using StarBooks.Domain.Books;

namespace StarBooks.Domain.Core.Converters;

public class AuthorConverter: JsonConverter<AuthorModel>
{
    public override AuthorModel? Read(ref Utf8JsonReader reader, Type typeToConvert, JsonSerializerOptions options)
    {
       return AuthorModel.Parse(reader.GetString()!);
    }

    public override void Write(Utf8JsonWriter writer, AuthorModel value, JsonSerializerOptions options)
    {
        writer.WriteStringValue(value.ToString());
    }
}