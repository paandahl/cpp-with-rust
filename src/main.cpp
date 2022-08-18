#include "multibuf.h"
#include "lib.rs.h"

#include <iostream>
#include <vector>

std::vector<uint8_t> get_bytes_from_string(std::string&& str) {
    return {str.begin(), str.end()};
}

int main() {
    auto client = org::blobstore::new_blobstore_client();

    // Upload a blob.
    auto chunks = std::vector {
            get_bytes_from_string("fearless"),
            get_bytes_from_string("concurrency")
    };
    auto buf = org::blobstore::MultiBuf { std::move(chunks), 0 };
    const auto blobid = client->put(buf);

    // Add a tag.
    client->tag(blobid, rust::String("rust"));

    // Read back the tags.
    const auto metadata = client->metadata(blobid);
    std::cout << "tags = { ";
    for (const auto& tag : metadata.tags) {
        std::cout << static_cast<std::string>(tag) << ' ';
    }
    std::cout << '}' << std::endl;

    return 0;
}
