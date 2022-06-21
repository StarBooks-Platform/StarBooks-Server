using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Microsoft.EntityFrameworkCore;

namespace StarBooks.Domain.Books;

[Table("accessInfos"), Owned]
public record AccessInfo(
    [property: JsonIgnore, Key] Guid Id,
    [property: JsonPropertyName("country")]
    string Country,
    [property: JsonPropertyName("viewability")]
    string Viewability,
    [property: JsonPropertyName("embeddable")]
    bool Embeddable,
    [property: JsonPropertyName("publicDomain")]
    bool PublicDomain,
    [property: JsonPropertyName("textToSpeechPermission")]
    string TextToSpeechPermission,
    [property: JsonPropertyName("epub")] EResource Epub,
    [property: JsonPropertyName("pdf")] EResource Pdf,
    [property: JsonPropertyName("webReaderLink")]
    string WebReaderLink,
    [property: JsonPropertyName("quoteSharingAllowed")]
    bool QuoteSharingAllowed
);
// {
//     private AccessInfo(
//         Guid id,
//         string country,
//         string viewability,
//         bool embeddable,
//         bool publicDomain,
//         string textToSpeechPermission,
//         string webReaderLink,
//         bool quoteSharingAllowed
//     ): this()
//     {
//         Id = id;
//         Country = country;
//         Viewability = viewability;
//         Embeddable = embeddable;
//         PublicDomain = publicDomain;
//         TextToSpeechPermission = textToSpeechPermission;
//         WebReaderLink = webReaderLink;
//         QuoteSharingAllowed = quoteSharingAllowed;
//     }
// }
